use view::View;

mod view;
mod screens;

fn main(){
    let mut view = View::new();
    loop {
        view.update();
    }
}