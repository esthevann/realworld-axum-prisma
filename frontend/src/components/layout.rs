use yew::prelude::*;
use super::{Header, Footer};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub children: Children, // the field name `children` is important!
}

#[function_component(Layout)]
pub fn layout(props: &Props) -> Html {
    html!{
        <>
            <Header />
                {for props.children.iter()}
            <Footer />
        </>
        
    }
}
