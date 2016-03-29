pub mod header;

pub struct Propfind {
    pub props: Vec<Prop>
}


pub struct Prop {
    pub name: String,
}
