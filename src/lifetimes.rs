struct Person<'a> {
    name: String,
    friends: Vec<&'a Person<'a>>
}
pub fn start() {

}
