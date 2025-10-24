#![allow(dead_code, unused_variables)]


fn main() {
    // naive();
    // typestates1();
    // typestates2();
}

fn naive() {
    // Quite often structs have some internal state. For instance, let's write a mock File object that 
    // opens a file from the filesystem and writes to it:

    struct FileWriter {
        filepath: String,
        is_open: bool,
    }
    impl FileWriter {
        fn new(filepath: String) -> FileWriter {
                FileWriter {filepath, is_open: false}
        }
        fn open(&mut self) {
            if self.is_open {
                panic!("Tried to open already open file!");
            }
            /* Open file */
            self.is_open = true;
        }
        fn write(&mut self, data: &str) {
            if !self.is_open {
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
    let mut f: FileWriter = FileWriter::new("abc123.txt".to_string());
    f.open();
    f.write("abc");
    f.close();
    f.close(); // Panic!

    // Notice that a lot of this code is error checking. In every function we have to be in the right state, 
    // otherwise things go wrong. 
}

fn typestates1() {
    struct FileWriterOpen{
        filepath: String
    }
    impl FileWriterOpen {
        fn write(&self, data: &str) {
            /* Open file */
        }
        fn close(self) -> FileWriterClosed {
            /* Close file */
            FileWriterClosed{filepath: self.filepath}
        }
    }
    struct FileWriterClosed {
        filepath: String
    }
    impl FileWriterClosed {
        fn new(filepath: String) -> FileWriterClosed {
            // Note: Compiler can infer the generic from return type
            FileWriterClosed{filepath}
        }
        fn open(self) -> FileWriterOpen {
            /* Open file */
            FileWriterOpen{filepath: self.filepath}
        }
    }

    let file_closed = FileWriterClosed::new("/var/lib/file.txt".to_string());
    let file_open = file_closed.open();
    //file_closed.open(); // Note: the open() method consumes FileClosed, so we can't wrongly access the old variable! 
    // This problem can't be fixed in most other languages!
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
    // It would be nice to combine FileOpen and FileClosed into a single type, while keeping the typestate information. 
    // If we add a generic bound we can do this!

    // We'll make some empty structs that represent state. Because they're empty they get completely compiled away.
    struct Open;
    struct Closed;

    // This is a secret tool that will help us in a second
    use std::marker::PhantomData;

    struct FileWriter<S>{ // Our struct has a generic that will be either the 'Open' or 'Closed' types above.
        filepath: String,
        // The compiler complains if we don't use our generic. PhantomData makes the compiler think we're using it.
        _state: PhantomData<S>, 
    }
    impl FileWriter<Closed> {
        fn new(filepath: String) -> FileWriter<Closed> {
            // Note: Compiler can infer the generic from return type
            FileWriter{filepath, _state: PhantomData}
        }
        fn open(self) -> FileWriter<Open> {
            /* Open file */
            FileWriter{filepath: self.filepath, _state: PhantomData}
        }
    }
    impl FileWriter<Open> {
        fn write(&self, data: &str) {
            /* Open file */
        }
        fn close(self) -> FileWriter<Closed> {
            /* Close file */
            FileWriter{filepath: self.filepath, _state: PhantomData}
        }
    }
           	 
    // Now if we use this File object:

    let f = FileWriter::new("abc123.txt".to_string());
    let f = f.write(); // Compile error! File<Closed> has no 'write()'
    let f = f.open();
    f.write("abc");
    let f = f.close();
    let f = f.close(); // Compile error! File<Closed> has no 'close()'
}
