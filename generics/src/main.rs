use generics:: {
    Book,
    Movie,
    print_description
};

fn main() {
    let mybook = Book { 
        title: String::from("Eragon"), 
        author: String::from("Some dude") 
    };

    let mymovie = Movie { 
        title: String::from("Transformers"), 
        director: String::from("Micheal Bay") 
    };

    print_description(&mymovie);
    print_description(&mybook);
}




