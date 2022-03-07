use crate::data;
use cursive::{
    event::Key::Esc,
    view::{Nameable, Resizable},
    views::{Button, Dialog, DummyView, EditView, LinearLayout, OnEventView, SelectView, TextView},
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

    siv.add_layer(Dialog::around(welcome).button("Back", user_list));
}

fn create_user(siv: &mut Cursive) {
    siv.add_layer(OnEventView::new(
        Dialog::around(
            LinearLayout::vertical()
                .child(TextView::new("Choose the name of the user"))
                .child(DummyView)
                .child(EditView::new().on_submit(|s, item| {
                    s.with_user_data(|data: &mut data::ProgramData| {
                        data.user_list
                            .insert(item.to_string(), User::new(item.to_string()))
                    });
                    s.pop_layer();
                    s.pop_layer();
                    user_list(s);
                })),
        )
        .title("Create a new user")
        .button("Back", |s| {
            s.pop_layer();
            user_list(s)
        }),
    ))
}

fn rename_user(siv: &mut Cursive) {
    siv.add_layer(
        OnEventView::new(
            Dialog::around(
                LinearLayout::vertical()
                    .child(TextView::new("Choose a new name"))
                    .child(DummyView)
                    .child(EditView::new().on_submit(|s, item| {
                        let selected = s
                            .call_on_name("user_select", |view: &mut SelectView| {
                                let (slug, _user) = view
                                    .get_item(
                                        view.selected_id()
                                            .expect("cannot get the ID of the selected item"),
                                    )
                                    .expect("cannot get selected item");

                                slug.to_string()
                            })
                            .expect("cannot call user_select");

                        s.with_user_data(|data: &mut data::ProgramData| {
                            let mut user = data
                                .user_list
                                .remove(&selected)
                                .expect("cannot remove user from user_list");

                            user.name = item.to_string();
                            data.user_list.insert(user.name.clone(), user)
                        });
                        s.pop_layer();
                        s.pop_layer();
                        user_list(s)
                    })),
            )
            .title("Rename a user")
            .button("Back", |s| {
                s.pop_layer();
            }),
        )
        .on_event(Esc, |s| {
            s.pop_layer();
        }),
    )
}

fn remove_user(siv: &mut Cursive) {
    let selected = siv
        .call_on_name("user_select", |view: &mut SelectView| {
            let (slug, _user) = view
                .get_item(view.selected_id().expect("cannot get selected id"))
                .expect("cannot get selected item");

            slug.to_string()
        })
        .expect("cannot call user_select");

    siv.add_layer(
        OnEventView::new(
            Dialog::around(TextView::new(format!(
                "You are going to delete {}",
                &selected
            )))
            .title("Delete a user")
            .button("Yes", move |s| {
                s.with_user_data(|data: &mut data::ProgramData| {
                    data.user_list.remove(&selected);
                });
                s.pop_layer();
                s.pop_layer();
                user_list(s);
            })
            .button("Back", |s| {
                s.pop_layer();
            }),
        )
        .on_event(Esc, |s| {
            s.pop_layer();
        }),
    );
}
