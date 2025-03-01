use rascii_art::{
    render_to,
    RenderOptions,
};
                                                            
fn main() {
    let mut buffer = String::new();
                                                            
    render_to(
        r"/mnt/Fedora2/code/AlbumMagic2/backend/image.png",
        &mut buffer,
        &RenderOptions::new()
            .width(40)
            .colored(true)
            .charset(&[" ", "░", "▒", "▓", "█"]),
    )
    .unwrap();

    println!("{}", buffer);
}
