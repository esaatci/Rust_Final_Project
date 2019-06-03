use std::{
    cell::RefCell,
    cmp::Ordering,
    fmt,
    hash::{Hash, Hasher},
    ops::Deref,
    rc::{Rc, Weak},
};

// Add weak-table 0.2 to your Cargo.toml
use weak_table::WeakHashSet;

/// A reference-counted symbol. Comparisons and hashing are constant
/// time.
#[derive(Clone)]
pub struct Symbol(Rc<str>);

/// Alias for [`Symbol::new`](struct.Symbol.html#method.new).
pub fn intern<N>(name: N) -> Symbol
where
    N: Into<String> + AsRef<str>,
{
    Symbol::new(name)
}

impl Symbol {
    /// Returns the interned symbol for the given name, by creating or
    /// retrieving it, as necessary.
    ///
    /// ## Examples
    ///
    /// ```
    /// # use lambda::symbol::Symbol;
    /// assert_eq!( Symbol::new("hello"), Symbol::new("hello") );
    /// assert_eq!( Symbol::new("hello"), Symbol::new("hello".to_owned()) );
    ///
    /// assert_ne!( Symbol::new("hello"), Symbol::new("goodbye") );
    /// ```
    pub fn new<N>(name: N) -> Self
    where
        N: Into<String> + AsRef<str>,
    {
        SymbolTable::with_thread_local_mut(|table| table.intern(name))
    }

    /// Returns an uninterned symbol different from all others. The name is used
    /// for printing but not comparison or hashing.
    ///
    /// ## Examples
    ///
    /// ```
    /// # use lambda::symbol::Symbol;
    /// assert_ne!( Symbol::uninterned("hello"), Symbol::uninterned("hello") );
    /// assert_ne!( Symbol::new("hello"),        Symbol::uninterned("hello") );
    /// ```
    pub fn uninterned(name: impl Into<String>) -> Self {
        Symbol(name.into().into())
    }

    /// Downgrades to a [`WeakSymbol`](struct.WeakSymbol.html), which may be freed
    /// if all the non-weak instances are dropped.
    pub fn downgrade(&self) -> WeakSymbol {
        WeakSymbol(Rc::downgrade(&self.0))
    }

    /// Gets the name of this symbol.
    ///
    /// ## Examples
    ///
    /// ```
    /// # use lambda::symbol::Symbol;
    /// assert_eq!( Symbol::new("hello").name(),       "hello" );
    /// assert_eq!( Symbol::uninterned("bees").name(), "bees" );
    /// ```
    pub fn name(&self) -> &str {
        &self.0
    }

    fn identity(&self) -> usize {
        self.0
            .as_bytes()
            .get(0)
            .map_or(0, |r| r as *const u8 as usize)
    }
}

impl From<String> for Symbol {
    fn from(name: String) -> Self {
        Symbol::new(name)
    }
}

impl From<&str> for Symbol {
    fn from(name: &str) -> Self {
        Symbol::new(name)
    }
}

impl PartialEq for Symbol {
    fn eq(&self, other: &Symbol) -> bool {
        Rc::ptr_eq(&self.0, &other.0)
    }
}

impl PartialEq<str> for Symbol {
    fn eq(&self, other: &str) -> bool {
        self.name() == other
    }
}

impl Eq for Symbol {}

impl PartialOrd for Symbol {
    fn partial_cmp(&self, other: &Symbol) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Symbol {
    fn cmp(&self, other: &Self) -> Ordering {
        self.identity().cmp(&other.identity())
    }
}

impl Hash for Symbol {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.identity().hash(state)
    }
}

impl Deref for Symbol {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.name()
    }
}

impl fmt::Debug for Symbol {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_tuple("Symbol").field(&self.name()).finish()
    }
}

impl fmt::Display for Symbol {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(self.name())
    }
}

/// A weak reference to a [`Symbol`](struct.Symbol.html). Because
/// symbols are reference counted, any reference to a symbol will keep
/// it from being dropped. A weak reference lets us refer to a symbol
/// without keeping it alive, and later we can try to convert back to
/// a `Symbol`, which will succeed if some other reference kept the
/// symbol alive.
///
/// ## Examples
///
/// ```
/// # use lambda::symbol::intern;
/// let strong = intern("name");
/// let weak = strong.downgrade();
///
/// assert!( !weak.is_expired() );
/// drop(strong);
/// assert!( weak.is_expired() );
/// ```
#[derive(Clone)]
pub struct WeakSymbol(Weak<str>);

impl WeakSymbol {
    /// Attempts to regain a strong reference from a weak symbol.
    pub fn upgrade(&self) -> Option<Symbol> {
        Weak::upgrade(&self.0).map(Symbol)
    }

    /// Returns whether this weak symbol is expired, meaning that
    /// attempting to upgrade it will fail.
    pub fn is_expired(&self) -> bool {
        self.upgrade().is_none()
    }
}

impl PartialEq for WeakSymbol {
    fn eq(&self, other: &WeakSymbol) -> bool {
        self.upgrade().map(|a| a == *other).unwrap_or(false)
    }
}

impl PartialEq<Symbol> for WeakSymbol {
    fn eq(&self, other: &Symbol) -> bool {
        self.upgrade().as_ref() == Some(other)
    }
}

impl PartialEq<WeakSymbol> for Symbol {
    fn eq(&self, other: &WeakSymbol) -> bool {
        Some(self) == other.upgrade().as_ref()
    }
}

impl From<&'_ Symbol> for WeakSymbol {
    fn from(symbol: &Symbol) -> Self {
        symbol.downgrade()
    }
}

impl fmt::Debug for WeakSymbol {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if let Some(sym) = self.upgrade() {
            f.debug_tuple("WeakSymbol").field(&sym.name()).finish()
        } else {
            f.write_str("WeakSymbol")
        }
    }
}

/// An explicitly-managed symbol table.
///
/// [`Symbol::new`](struct.Symbol.html#method.new) interns the symbol
/// in a thread-local (if your program is single-threaded, that means
/// effectively global) table. It's also possible to manage tables
/// explicitly using this type. This allows you to have multiple
/// interned symbols with the same name that aren’t equal because
/// they’re interned in separate tables.
#[derive(Default)]
pub struct SymbolTable(WeakHashSet<Weak<str>>);

impl SymbolTable {
    /// Calls the given closure with a shared reference to the thread
    /// local symbol table.
    pub fn with_thread_local<F, R>(f: F) -> R
    where
        F: FnOnce(&SymbolTable) -> R,
    {
        SYMBOL_TABLE.with(|c| f(&c.borrow()))
    }

    /// Calls the given closure with a mutable reference to the thread
    /// local symbol table.
    pub fn with_thread_local_mut<F, R>(f: F) -> R
    where
        F: FnOnce(&mut SymbolTable) -> R,
    {
        SYMBOL_TABLE.with(|c| f(&mut c.borrow_mut()))
    }

    /// Creates a new symbol table.
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates a new symbol with some preallocated capacity.
    pub fn with_capacity(capacity: usize) -> Self {
        SymbolTable(WeakHashSet::with_capacity(capacity))
    }

    /// Interns a symbol in a specified table.
    pub fn intern<N>(&mut self, name: N) -> Symbol
    where
        N: Into<String> + AsRef<str>,
    {
        Symbol(self.insert(name))
    }

    fn insert<N>(&mut self, name: N) -> Rc<str>
    where
        N: Into<String> + AsRef<str>,
    {
        self.0.get(name.as_ref()).unwrap_or_else(|| {
            let rc: Rc<str> = name.into().into();
            self.0.insert(rc.clone());
            rc
        })
    }
}

thread_local! {
    static SYMBOL_TABLE: RefCell<SymbolTable> = RefCell::new(SymbolTable::new());
}
