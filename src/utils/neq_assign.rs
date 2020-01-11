use yew::ShouldRender;

pub trait NeqAssign {
    fn neq_assign(&mut self, new: Self) -> ShouldRender;
}

impl<T: PartialEq> NeqAssign for T {
    fn neq_assign(&mut self, new: T) -> ShouldRender {
        if self != &new {
            *self = new;
            true
        } else {
            false
        }
    }
}
