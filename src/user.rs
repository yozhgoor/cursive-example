use crate::data;
use cursive::{
    event::Key::Esc,
    view::{Nameable, Resizable},
    views::{Button, Dialog, DummyView, LinearLayout, OnEventView, SelectView, TextView},
    Cursive,
};

#[derive(Debug)]
pub struct User {
    pub name: String,
}

impl User {
    pub fn new(name: String) -> Self {
        Self { name }
    }
}

pub fn user_list(siv: &mut Cursive) {
    let user_list = &siv
        .user_data::<data::ProgramData>()
        .expect("cannot get program data")
        .user_list;

    let user_select = SelectView::new()
        .with_all_str(user_list.keys())
        .on_submit(|s, item: &str| {
            s.with_user_data(|data: &mut data::ProgramData| {
                data.active_user = Some(item.to_string());
            });
            s.pop_layer();
            user_welcome(s);
        })
        .with_name("user_select")
        .fixed_size((60_usize, 20_usize));

    let select_buttons = LinearLayout::vertical()
        .child(Button::new("Create", create_user))
        .child(Button::new("Rename", rename_user))
        .child(Button::new("Remove", remove_user));

    let list_frame = LinearLayout::horizontal()
        .child(user_select)
        .child(DummyView)
        .child(select_buttons);

    siv.add_layer(
        OnEventView::new(
            Dialog::around(list_frame)
                .title("Select a user")
                .button("Quit", |s| s.quit()),
        )
        .on_event(Esc, |s| s.quit()),
    );
}

pub fn user_welcome(siv: &mut Cursive) {
    let data = siv
        .user_data::<data::ProgramData>()
        .expect("cannot get program data");

    let user = data
        .user_list
        .get(&data.active_user.clone().expect("no active user"))
        .expect("user not found in user list");

    let welcome = TextView::new(format!("Hello {}!", user.name));

    siv.add_layer(
        Dialog::around(welcome)
            .button("Back", user_list)
    );
}

fn create_user(_siv: &mut Cursive) {
    unimplemented!()
}

fn rename_user(_siv: &mut Cursive) {
    unimplemented!()
}

fn remove_user(_siv: &mut Cursive) {
    unimplemented!()
}
