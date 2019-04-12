pub trait StateOp {
    type State;

    fn apply(&self, state: &mut Self::State) -> Result<(), ()>;
}

pub trait Journal {
    type Index;
    type Op: StateOp<State = Self::State> + Copy;
    type State;

    fn append_entry(&mut self, op: &Self::Op) -> Result<(), ()>;
    fn get_entry(&self, idx: Self::Index) -> Result<&Self::Op, ()>;
    fn next_index(&self) -> Self::Index;
    fn state(&mut self) -> &Self::State;
}

#[cfg(test)]
mod test {
    use crate::journal::{Journal, StateOp};

    #[derive(Copy, Clone, Debug)]
    enum CounterOp<T>
    where
        T: Copy,
    {
        Increment(T),
        Decrement(T),
    }

    impl<T> CounterOp<T>
    where
        T: Copy,
    {
        fn inc(x: T) -> Self {
            CounterOp::Increment(x)
        }
        fn dec(x: T) -> Self {
            CounterOp::Decrement(x)
        }
    }

    use std::ops::{AddAssign, SubAssign};

    impl<T> StateOp for CounterOp<T>
    where
        T: AddAssign + SubAssign + Copy,
    {
        type State = T;

        fn apply(&self, state: &mut T) -> Result<(), ()> {
            match *self {
                CounterOp::Increment(inc) => *state += inc,
                CounterOp::Decrement(dec) => *state -= dec,
            }
            Ok(())
        }
    }

    struct CounterJournal {
        log: Vec<CounterOp<i64>>,
        state: i64,
    }

    impl CounterJournal {
        fn new() -> CounterJournal {
            CounterJournal {
                log: vec![],
                state: 0,
            }
        }
    }

    impl Journal for CounterJournal {
        type Index = usize;
        type Op = CounterOp<i64>;
        type State = i64;

        fn next_index(&self) -> Self::Index {
            self.log.len()
        }
        fn state(&mut self) -> &Self::State {
            &self.state
        }

        fn get_entry(&self, idx: Self::Index) -> Result<&Op, ()> {
            self.log.get(idx).ok_or(())
        }

        fn append_entry(&mut self, op: &Self::Op) -> Result<(), ()> {
            op.apply(&mut self.state)?;
            self.log.push(*op);
            Ok(())
        }
    }

    #[test]
    fn state_ops() {
        let mut state: i64 = 0;
        let ops = vec![CounterOp::inc(3), CounterOp::dec(2)];
        assert_eq!(state, 0);
        ops[0].apply(&mut state).expect("apply [0] failed");
        assert_eq!(state, 3);
        ops[1].apply(&mut state).expect("apply [0] failed");
        assert_eq!(state, 1);
    }

    #[test]
    fn journal_ops() {
        let mut journal = CounterJournal::new();
        let ops = vec![CounterOp::inc(3), CounterOp::dec(2)];
        assert_eq!(*journal.state(), 0);
        journal.append_entry(&ops[0]).expect("apply [0] failed");
        assert_eq!(*journal.state(), 3);
        journal.append_entry(&ops[1]).expect("apply [0] failed");
        assert_eq!(*journal.state(), 1);
    }
}
