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
    // otherwise things go wrong. 
}

fn typestates1() {
    struct FileOpen{
        filepath: String
    }
    impl FileOpen {
        fn write(&self, data: &str) {
            /* Open file */
        }
        fn close(self) -> FileClosed {
            /* Close file */
            FileClosed{filepath: self.filepath}
        }
    }
    struct FileClosed {
        filepath: String
    }
    impl FileClosed {
        fn new(filepath: String) -> FileClosed {
            // Note: Compiler can infer the generic from return type
            FileClosed{filepath}
        }
        fn open(self) -> FileOpen {
            /* Open file */
            FileOpen{filepath: self.filepath}
        }
    }

    let file_closed = FileClosed::new("/var/lib/file.txt".to_string());
    let file_open = file_closed.open();
    //file_closed.open(); // Note: the open() method consumes FileClosed, so we can't wrongly access the old variable! This problem can't be fixed in most other languages!
    // And even better, Rust supports 'variable shadowing' so we can actually just call them the same thing:
    let file = file_open.close();
    let file = file.open();
    file.write("abcd");
    let file = file.close();
    file.write(); // Compile error! No write() method for FileClosed!

    // And with that, we've just moved an entire class of runtime errors into compile-time errors!

    // We saw in the generics example that generics can be used to add methods to
    // particular subtypes of a struct. Could we use this to encode the state of an object into its subtype? Yes!
}

fn typestates2() {
    // It would be nice to combine FileOpen and FileClosed into a single type, while keeping the typestate information. If we add a generic bound we can do this!

    // We'll make some empty structs that represent state. Because they're empty they get completely compiled away.
    struct Open;    
    struct Closed;    

    // This is a secret tool that will help us in a second
    use std::marker::PhantomData;

    struct File<S>{ // Our struct has a generic that will be either the 'Open' or 'Closed' types above.
        filepath: String,
        // The compiler complains if we don't use our generic. PhantomData tells the compiler we don't actually need it.
        _state: PhantomData<S>, 
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
        fn write(&self, data: &str) {
            /* Open file */
        }
        fn close(self) -> File<Closed> {
            /* Close file */
            File{filepath: self.filepath, _state: PhantomData}
        }
    }
           	 

    // Now if we use this File object:

    let f = File::new("abc123.txt".to_string());
    let f = f.write(); // Compile error! File<Closed> has no 'write()'
    let f = f.open();
    f.write("abc");
    let f = f.close();
    let f = f.close(); // Compile error! File<Closed> has no 'close()'
}
