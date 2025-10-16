#![allow(dead_code, unused_variables)]


fn main() {
    // naive();
    // typestates();
}

fn naive() {
    // Quite often structs have some internal state. For instance, let's write a mock File object that 
    // opens a file from the filesystem and writes to it:

    struct File {
        filepath: String,
        is_open: bool,
    }
    impl File {
        fn new(filepath: String) -> File {
                File {filepath, is_open: false}
        }
        fn open(&mut self) {
            if self.is_open {
                panic!("Tried to open already open file!");
            }
            /* Open file */
            self.is_open = true;
        }
        fn write(&mut self, data: &str) {
            if !self.is_open{
                panic!("Tried to write to closed file!");
            }
            /* write to file */
        }
        fn close(&mut self) {
            if !self.is_open {
                panic!("Tried to close already closed file!");
            }
            /* Close file */
            self.is_open = false;
        }
    }
    let mut f: File = File::new("abc123.txt".to_string());
    f.open();
    f.write("abc");
    f.close();
    f.close(); // Panic!

    // Notice that a lot of this code is error checking. In every function we have to be in the right state, 
    // otherwise things go wrong. We saw in the previous example that generics can be used to add methods to
    // particular subtypes of a struct. Could we use this to encode the state of an object into its type? Yes!
}

fn typestates() {
    use std::marker::PhantomData;
    struct File<State>{ // We make our own FileState trait
        filepath: String,
        // The compiler complains if we don't use our generic 
        _state: PhantomData<State>, 
    }
    impl File<Closed> {
        fn new(filepath: String) -> File<Closed> {
            // Note: Compiler can infer the generic from return type
            File{filepath, _state: PhantomData}
        }
        fn open(self) -> File<Open> {
            /* Open file */
            File{filepath: self.filepath, _state: PhantomData}
        }
    }
    impl File<Open> {
        fn write(&mut self, data: &str) {
            /* Open file */
        }
        fn close(self) -> File<Closed> {
            /* Close file */
            File{filepath: self.filepath, _state: PhantomData}
        }
    }
    struct Open;    // And some empty structs...
    struct Closed;           	 

    // Now if we use this File object:

    let f = File::new("abc123.txt".to_string());
    let f = f.write(); // Compile error! File<Closed> has no 'write()'
    let f = f.open();
    f.write("abc");
    let f = f.close();
    let f = f.close(); // Compile error! File<Closed> has no 'close()'

    // And with that, we've just moved an entire class of runtime errors into compile-time errors!
}
