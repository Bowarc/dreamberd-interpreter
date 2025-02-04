// Import from the memory module
struct Scope;

pub struct Scopes{
    global: Scope,
    stack: Vec<Scope>
}
