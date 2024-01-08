#[macro_use]
extern crate rocket;

use rocket::tokio::sync::broadcast::channel;

#[get("/")]
fn home() -> &'static str {
    "Hello World"
}

#[derive(Debug, Clone, FromForm, Serialize, Deserialise)]
#[serde(crate = "rocket::serde")]

struct Message {
    #[field(validate = len(..30))]
    pub room: String,
    #[field(validate = len(..20))]
    pub username: String,
    pub message: String,
}

#[post("/message", data = "<form>")]
fn post(form: Form<Message> queue: &State<Sender<Message>>) {
    let _res = queue.send(form.into_inner());
}

#[get("/events")]
async fn events(queue: &State<Sender<Message>>, mut end: Shutdown) ->EventStream![] {
    let mut rx = queue.subscribe();

    EventStream! {
        loop {
            let msg = select! {
                msg = rx.recv() => match msg {
                    Ok(msg) => msg,
                    Err(RecvError::Closed) => break,
                    Err(RecvError::Lagged(_)) => continue
                },
                _ = &mut end => break,
            };
            yield Event::json(&msg);
        }
    }
}
 
#[launch]
fn rocket() -> _ {
    rocket::build()
        .manage(channel::<Message>(1024).0)
        .mount("/", routes![home, post, events])
}