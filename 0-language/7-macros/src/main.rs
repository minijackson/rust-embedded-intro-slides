use std::convert::TryFrom;
use std::io::Read;

struct Memory(Vec<i32>);
struct Pointer(usize);

fn increment(pointer: &Pointer, mem: &mut Memory) {
    mem.0[pointer.0] += 1;
}

fn decrement(pointer: &Pointer, mem: &mut Memory) {
    mem.0[pointer.0] -= 1;
}

fn move_forward(pointer: &mut Pointer, mem: &mut Memory) {
    pointer.0 += 1;
    if pointer.0 >= mem.0.len() {
        mem.0.resize(pointer.0 + 1, 0);
    }
}

fn move_backward(pointer: &mut Pointer) {
    pointer.0 -= 1;
}

fn print(pointer: &Pointer, mem: &Memory) {
    let byte = u8::try_from(mem.0[pointer.0]).unwrap();
    print!("{}", char::from(byte));
}

#[allow(unused)]
fn get_char(pointer: &Pointer, mem: &mut Memory) {
    let mut res = [0u8; 1];
    std::io::stdin().read_exact(&mut res).unwrap();
    mem.0[pointer.0] = i32::from(res[0]);
}

macro_rules! brainfuck {
    ($($tt: tt)*) => {
        let mut mem = Memory(vec![0]);
        let mut pointer = Pointer(0);

        brainfuck_internal!(pointer, mem, $($tt)*);
    };
}

macro_rules! brainfuck_internal {
    ($pointer: ident, $mem: ident, + $($tt: tt)*) => {
        increment(&$pointer, &mut $mem);
        brainfuck_internal!($pointer, $mem, $($tt)*);
    };

    ($pointer: ident, $mem: ident, - $($tt: tt)*) => {
        decrement(&$pointer, &mut $mem);
        brainfuck_internal!($pointer, $mem, $($tt)*);
    };

    ($pointer: ident, $mem: ident, > $($tt: tt)*) => {
        move_forward(&mut $pointer, &mut $mem);
        brainfuck_internal!($pointer, $mem, $($tt)*);
    };

    // Tokenizer mishap
    ($pointer: ident, $mem: ident, -> $($tt: tt)*) => {
        brainfuck_internal!($pointer, $mem, - > $($tt)*);
    };

    // Tokenizer mishap
    ($pointer: ident, $mem: ident, >> $($tt: tt)*) => {
        brainfuck_internal!($pointer, $mem, > > $($tt)*);
    };

    ($pointer: ident, $mem: ident, < $($tt: tt)*) => {
        move_backward(&mut $pointer);
        brainfuck_internal!($pointer, $mem, $($tt)*);
    };

    // Tokenizer mishap
    ($pointer: ident, $mem: ident, <- $($tt: tt)*) => {
        brainfuck_internal!($pointer, $mem, < - $($tt)*);
    };

    // Tokenizer mishap
    ($pointer: ident, $mem: ident, << $($tt: tt)*) => {
        brainfuck_internal!($pointer, $mem, < < $($tt)*);
    };

    ($pointer: ident, $mem: ident, . $($tt: tt)*) => {
        print(&$pointer, &$mem);
        brainfuck_internal!($pointer, $mem, $($tt)*);
    };

    // Tokenizer mishap
    ($pointer: ident, $mem: ident, .. $($tt: tt)*) => {
        brainfuck_internal!($pointer, $mem, . . $($tt)*);
    };

    ($pointer: ident, $mem: ident, , $($tt: tt)*) => {
        get_char(&$pointer, &mut $mem);
        brainfuck_internal!($pointer, $mem, $($tt)*);
    };

    ($pointer: ident, $mem: ident, [ $($loop:tt)* ] $($tt: tt)*) => {
        loop {
            brainfuck_internal!($pointer, $mem, $($loop)*);
            if $mem.0[$pointer.0] == 0 {
                break;
            }
        }
        brainfuck_internal!($pointer, $mem, $($tt)*);
    };

    ($pointer: ident, $mem: ident, $comment: tt $($tt: tt)*) => {
        // Anything not brainfuck is a comment
        brainfuck_internal!($pointer, $mem, $($tt)*);
    };

    ($pointer: ident, $mem: ident,) => {
        // Finished!
    };
}

fn main() {
    brainfuck!(
        ++++++++
        [
            >++++
            [
                >++>+++>+++>+<<<<-
            ]
            >+>+>->>+
            [<]
            <-
        ]
        >>.>---.+++++++..+++.>>.<-.<.+++.------.--------.>>+.>++.
    );
}

// macro_rules!: declarative macros for function-like macros
// procedural macros allows:
//   - Function-like macros
//   - Derives
//   - Attribute-like macros
