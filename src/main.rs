use std::ffi::CString;
use std::panic::resume_unwind;
use std::process::exit;
use crate::LibraryError::AlreadyCheckedOut;
// This is the enum I am going to use to return an error into the Result .
pub enum LibraryError {
    NotFound(String),
    AlreadyCheckedOut(String),
    NotCheckedOut(String)
}
//This is the struct for Book which is like a Java class, but it doesn't hold any behavior,
// those will go into the impl
pub struct Book {
    title: String,
    author: String,
    borrower: Option<String>
}

//This is where all of the methods for Book will go.
impl Book {
    // This is a basic constructor. Call this when you want to make a new book, you are
    //required to tell it the author and title. When a new book is made it will have no borrower.
    //Needs to take in a &str because the caller is giving us a reference to a String which we will
    // then make into a String with .to_string. If we passed in a String, when we call Book::new
    //we would have to pass in "Dune".to_string(). This keeps it cleaner in the calling.
    pub fn new(title: &str, author: &str) -> Book {
        Book {
            title: title.to_string(),
            author: author.to_string(),
            borrower: None
        }
    }

    //Takes in &self and checks if the borrower field is None, if so the book is available.
    pub fn is_available(&self) -> bool {
        //Like lisp, the last line gets returned, so in most languages I would need an if else statement
        //Here I don't need one because it will just return true or false.
        self.borrower == None
    }

}

//Same as before this is like making a new class of Library
pub struct Library {
    //This is saying that in Library there is going to be the variable books which is going to be
    // a vector of Book. A vector is an array that can grow and shrink in size. Like an Array List
    books: Vec<Book>
}

//And here is where the behavior of Library is going to be.
impl Library {
    //Makes a new Library instance. where you can put Books in the new Vec that is created.
    pub fn new() -> Library {
        Library {
            books: Vec::new()
        }
    }

    //This will change the library but we don't want it to own the library,
    //so that is why the &mut. we do want the library to own the book so that
    //doesn't get a reference to it.
    pub fn add_book(&mut self, book: Book) {
        //This is adding the instance of Book that you pass in.
        self.books.push(book)
    }

    //This function loops over all of the books and prints out all of the books.
    //Takes a reference to itself so it can see all of the Books in the books Vec.
    pub fn list_books(&self){
        //check if there are no books in the library.
        if self.books.is_empty() {
            println!("There are no books in the library");
        }
        //Looping through the Vec of Book.
        //it is a & because it just needs to read about the Book not have the function take ownership of it
        for book in &self.books {
            //This is the first match statement and these come in very handy when dealing with the Option enum
            //match statements need to be extensive, so every different value that could happen needs to be written out
            //this will look at the borrower field on the Book and Option can either be Some, which will then hold a value
            //or it can be none, Rust has no "Null" it just isn't a thing that exists.
            //I think that it has to do with not having Null Pointer Exceptions.
            match &book.borrower {
                //if it matches on Some, it takes in the value it will print the line after it.
                //this syntax is something that I have seen a lot, the first {} will map to book.title
                // and the second {} will map to value which will be who has it checked out.
                Some(value) => println!("{} is Checked out by {}", book.title,value),
                //if the borrower field is None, it will print this.
                None => println!("Book is on the shelf")
            }
            //this just checks if the book is available and prints the title if it is.
            if book.is_available() {
                println!("{} is available", book.title);
            }
            //else { println!("{} is not available", book.title); }
        }
    }

    //should take in a books title and print everything about it.
    //Takes in a reference to the Library and title.
    pub fn find_books_by_title(&self, title: &str) {
        // loops through the Vec
        for books in &self.books {
            //if the title = the title that was passed in print info about it.
            if books.title == title {
                println!("Title: {}, Author: {}",books.title, books.author);
            }
        }
    }

    pub fn check_out_book(&mut self, title: String, who: String) -> Result<(String), LibraryError>{
        //somehow need to Change borrowed from None to Some.
        for book in &mut self.books {
            if book.title == title {
                if !book.is_available() {
                    return Err(LibraryError::AlreadyCheckedOut(title));
                } else {
                    //&mut book.borrower = &mut Option::Some(who)
                    book.borrower = Some(who);
                    return Ok((title))
                }
            }
        }
        Err(LibraryError::NotFound(title))
    }

    pub fn return_book(&mut self, title: String) -> Result<(String), LibraryError> {
        for book in &mut self.books {
            if book.title == title {
                if book.is_available(){
                    return Err(LibraryError::NotCheckedOut(title));
                }
                else {
                    book.borrower = None;
                    return Ok((title));
                }
            }
        }
        Err(LibraryError::NotFound(title))
    }

}

//Main function where everything gets set up.
fn main() {
    let mut library = Library::new();
    //let dune = Book::new("Dune", "Frank Herbert");
   // library.add_book(dune);
    library.list_books();

    //This is all taking the result from check_out_books and deciding what to print based on what gets returned
   let result_checkout = library.check_out_book("Dune".to_string(), "Damon".to_string());
    //This statements takes what was returned in
    match result_checkout{
        Ok(title) => println!("{} Checked out successfully!!", title),
        Err(LibraryError::NotFound(title)) => println!("{} is not found", title),
        Err(LibraryError::AlreadyCheckedOut(title)) => println!("{} is already checked out", title),
        Err(LibraryError:: NotCheckedOut(title)) => println!("{} is not checked out", title)
    }

    library.list_books();

    let result_return = library.return_book("Dune".to_string());
    match result_return {
        Ok((title)) => println!("{} has been returned successfully!!", title),
        Err(LibraryError::NotFound(title)) => println!("{} was not found", title),
        Err(LibraryError::NotCheckedOut(title)) => println!("you never had {} checked out", title),
        //This one should never happen but good to have anyways I guess.
        Err(LibraryError::AlreadyCheckedOut(title)) => println!("{} Is already checked out", title)
    }

}