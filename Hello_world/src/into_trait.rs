struct Person
{
    name: String
}

impl Person
{
    //fn new(name: &str) -> Person
    //{
   //     Person {name: name.to_string()}
  //  }
    //fn new<S: Into<String>>(name:S) -> Person
    fn new<S>(name: S) -> Person
     where S: Into<String>
    {
        Person{name: name.into()}
    }
}


pub fn into_trait()
{
    let john = Person::new("John");
    let name: String = "Jane".to_string();

    let jane = Person::new(name);
}