use std::collections::HashMap;

fn arrays() {
    //EXAMPLE: Moving
    let s = vec!["udon".to_string(), "noodles".to_string()];
    //Need to use clone since this is a move
    let _t = s.clone();
    let _u = s.clone();

    //EXAMPLE: Moving with Vectors
    let mut v = Vec::new();
    for i in 101..106 {
        v.push(i.to_string());
    }
    let fifth = v.pop().expect("vector empty!");
    assert_eq!(fifth, "105");

    //EXAMPLE: Iterate over a vector
    let v = vec!["A".to_string(),
                 "B".to_string(),
                 "C".to_string()];
    for mut s in v {
        s.push('!');
        println!("{}", s);
    }
}

type Table = HashMap<String, Vec<String>>;

fn show(table: &Table) {
    for (artist, works) in table {
        println!("works by {}:", artist);
        for work in works {
            println!("    {}", work);
        }
    }
}

fn sort(table: &mut Table) {
    for (_artist, works) in table {
        works.sort();
    }
}

fn hashmaps() {
    let mut table = Table::new();
    table.insert("A".to_string(),
                 vec![
                     "2".to_string(),
                     "1".to_string()
                 ]);
    table.insert("B".to_string(),
                 vec![
                     "2".to_string(),
                     "1".to_string()
                 ]);
    sort(&mut table);
    show(&table);
}

fn change_via_ref_de_referencing() {
    let mut y = 32;
    let m = &mut y;
    *m += 32;
    assert_eq!(*m, 64);
}

fn implicitly_de_reference() {
    struct Anime { name: &'static str, _bechdel_pass: bool }
    ;
    let aria = Anime { name: "something", _bechdel_pass: true };
    let anime_ref = &aria;
    assert_eq!(anime_ref.name, "something");

    assert_eq!((*anime_ref).name, "something");
}

fn borrowing_a_local_variable() {
    let x = 1;
    {
        let r = &x;
        assert_eq!(*r, 1);
    }
}

static mut STASH: &i32 = &10;

fn play_with_statics(){
    fn f(p: &'static i32){
        unsafe {
            STASH = p
        }
    }

    // Cannot do this as anything passed to f will live as long as the static
    // which means the ref to x will live longer than x
    // which is not allowed
    //
    // let x = 10;
    // f(&x);
    f(&1000);
    unsafe{
        println!("{}", STASH);
    }
}

fn play_with_returning_references(){
    fn smallest(v :&[i32]) -> &i32{
        assert!(v.len() > 0);
        let mut s = &v[0];
        for r in &v[1..]{
            if *r < *s { s = r};
        }
        s
    }

    let values = [1,2,3,4,5,6];
    let s = smallest(&values);
    assert_eq!(*s, 1);

    /*
    NB:// b will not live long enough since the implicit lifetime is 'a

    let b:&i32;
    {
        let more_values = [2,3,4];
        b = smallest(&more_values);
    }
    assert_eq!(*b, 2);
    */
}

fn play_with_lifetimes_with_structs(){
    struct S<'a, 'b>{
        x: &'a i32,
        y: &'b i32
    }

    let x = 10;
    let r;
    {
        let y = 20;
        {
            let s = S { x :&x, y: &y };
            r  = s.x;
        }
    }
}

fn play_with_structs_self(){
    struct StringTable{
        elements: Vec<String>,
    }
    impl StringTable{
        fn find_by_prefix(&self, prefix: &str) -> Option<&String>{
            for i in 0 .. self.elements.len(){
                if self.elements[i].starts_with(prefix){
                    return Some(&self.elements[i]);
                }
            }
            None
        }
    }

    let s = StringTable{
        elements: vec![
            "Joe".to_string(),
            "Andy".to_string(),
            "Jason".to_string()
        ],
    };

    let item = match s.find_by_prefix("And"){
        Some(value) => value,
        None => ""
    };
    println!("Found string with prefix: {}", item);
}

fn play_with_some(){
    fn something() -> Option<i32>{
        Some(1)
    }

    if let Some(val) = something(){
        println!("We have dodged the match to check for some {}", val)
    }
}

fn play_with_unwrap_or(){

}

fn main() {
    arrays();
    hashmaps();
    change_via_ref_de_referencing();
    implicitly_de_reference();
    borrowing_a_local_variable();
    play_with_statics();
    play_with_returning_references();
    play_with_lifetimes_with_structs();
    play_with_structs_self();
    play_with_some();
    println!("Hello, world!");
}
