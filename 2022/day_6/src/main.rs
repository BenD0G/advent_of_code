use input::INPUT;
mod input;

struct CircularBuffer<const N: usize> {
    buffer: [u32; N],
    index: usize,
}

impl<const N: usize> CircularBuffer<N> {
    fn new() -> Self {
        CircularBuffer {
            buffer: [0; N],
            index: 0,
        }
    }

    fn add_c(&mut self, c: char) {
        self.buffer[self.index] = c as u32;
        self.index = (self.index + 1).rem_euclid(N);
    }

    fn all_different(&self) -> bool {
        for i in 0..(N - 1) {
            for j in (i + 1)..N {
                if self.buffer[i] == self.buffer[j] {
                    return false;
                }
            }
        }
        true
    }
}

fn solve<const N: usize>() -> usize {
    let mut buf = CircularBuffer::<N>::new();
    for (i, c) in INPUT.chars().enumerate() {
        buf.add_c(c);
        if i > (N - 2) && buf.all_different() {
            return i + 1;
        }
    }
    69
}

fn main() {
    println!("{}", solve::<4>());
    println!("{}", solve::<14>());
}
