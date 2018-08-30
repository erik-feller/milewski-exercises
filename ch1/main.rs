fn main() {

  //Test the id function
  let x = 6;
  let y = id(x);
  if x == y {
      println!("The id function works (at least for integers)");
  }

  //Exercise 3 (cont.)
  let on_arg = comp(id(x), &add3, &is_odd);
  let on_comp = id(comp(x, &add3, &is_odd));
  if on_arg == on_comp {
      println!("This simple test passed! That means its bulletproof!");
  }
}

//Exercise 1
fn id<T> (x: T) -> T{
    x  
}

//Exercise 2
//I'm not super happy that you have to supply an argument for this
//But to be honest I don't have a good grasp on rust trait objects
//yet
fn comp<A, B, C> (arg:A, f: &Fn(A) -> B, g: &Fn(B) -> C) -> C {
    g(f(arg))
}

//For Exercise 3
//Conceptually all we need is for id(comp(i)) == comp(id(i)) I think
fn add3 (i:i32) -> i32 {
    i + 3
}

fn is_odd (i:i32) -> bool {
    if i%2 == 0 {
        return false;
    } else {
        return true;
    }
}

//Exercise 4
//The world wide web as a greater whole can't really be considered a 
//category if you think of pages as objects and links as morphisms because
//if a link from page A goes to page B and a link from B goes to C there is
//not necessarily a link from A to C. I can't think of a different logical way
//to represent the web as a category

//Exercise 5
//The facebook group scenario is very similar to the web one in its pitfalls.
//Since if person A is friends with person B and person B is friends with person C
//A may not necessarily be friends with person C the definition of a category does
//not apply.

//Exercise 6
//A directed graph is a category when each node is directly connected to each other node. Unless
//the graph doesn't have any loops then it is possible for only nodes higher up to be connected to
//every node. For example if the A -> B -> C -> D is the basic structure you would also need A -> C
//and A -> D to be edges. B-> D is also needed. These are just two simple examples but there may be
//others
