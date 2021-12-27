mod lib_macros{
	#[macro_export]
	/// Create a **VecDeque** from a list of values
	///
	/// ## Example
	///
	/// ```
	/// #[macro_use];
	/// # fn main() {
	///
	/// let deq = vecdeque!{
	///     1,
	///     2,
	///     3,
	/// };
	/// assert_eq!(deq.len(), 3);
	/// assert_eq!(deq.get(0), Some(&1));
	/// assert_eq!(deq.get(1), Some(&2));
	/// assert_eq!(deq.get(2), Some(&3));
	/// assert_eq!(deq.get(3), None);
	/// # }
	/// ```
	///
	/// ```
	/// #[macro_use];
	///
	/// let deq = vecdeque![3; 5];
	/// assert_eq!(deq.len(), 5);
	/// assert_eq!(deq.get(0), Some(&3));
	/// assert_eq!(deq.get(4), Some(&3));
	/// assert_eq!(deq.get(5), None);
	/// ```
	macro_rules! vecdeque {
    (@single $($x:tt)*) => (());
    (@count $($rest:expr),*) => (<[()]>::len(&[$(vecdeque!(@single $rest)),*]));

    ($($value:expr,)+) => { vecdeque!($($value),+) };
    ($($value:expr),*) => {
        {
            let _cap = vecdeque!(@count $($value),*);
            let mut _map = ::std::collections::VecDeque::with_capacity(_cap);
            $(
                _map.push_back($value);
            )*
            _map
        }
    };
    ($value:expr;$count:expr) => {
        {
            let c = $count;
            let mut _map = ::std::collections::VecDeque::with_capacity(c);
            for _ in 0..c {
                _map.push_back($value);
            }
            _map
        }
    };
	}
}