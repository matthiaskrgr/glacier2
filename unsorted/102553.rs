pub trait Widget<E> {
    fn boxed<'w>(self) -> Box<dyn WidgetDyn<E>+'w> where Self: Sized + 'w;
}

pub trait WidgetDyn<E> {}

impl<T,E> WidgetDyn<E> for T where T: Widget<E> {}

impl<E> Widget<E> for dyn WidgetDyn<E> + '_ {
    fn boxed<'w>(self) -> Box<dyn WidgetDyn<E>+'w> where Self: Sized + 'w {
        Box::new(self) //ICE: cannot copy unsized immediates
    }
}

pub fn main() {}
