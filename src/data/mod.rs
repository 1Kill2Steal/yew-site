// You need to have some image pattern alongside the image extension.
pub static IMAGE_NAME_PATTERN: &str = "Hu_Tao_";
pub static COMPRESSED_IMAGE_EXTENSION: &str = ".jpg";
pub static UNCOMPRESSED_IMAGE_EXTENSION: &str = ".png";

// You need to change this if you want to have your images in a different directory
// NOTE: If you do change it you also need to change it in /index.html where all the data trunks
// are copied over.
pub static PICS_COMPRESSED_FOLDER_NAME: &str = "hutao/pics/";
pub static PICS_UNCOMPRESSED_FOLDER_NAME: &str = "hutao/pics_uncompressed/";

// Unfortunately you need to check and set those up yourself. This is a Trunk limitation since
// counting the folder items inside the trunk serve is impossible. And making a utility script that
// does that will make it so you need to enter the target folders twice so it ends up not being
// that much more friendly anyway.
//
// WARN: It's important that you make sure the folders are actually the same in size!!
//
// The reason is because there's download buttons for the images that correspond to the
// same image in the different directory.
// "Why?" - The displayed images are quality 70 JPGs while (most of) the download option ones are
// max quality PNGs. Of course you can always have 2 identical copies if you don't care about
// bandwidth OR you can change the layout to only use one folder. Just don't complain when you get
// negative reviews about your page having slow load times.
pub static COMPRESSED_PICS_FOLDER_SIZE: usize = 150;
#[allow(dead_code)]
pub static UNCOMPRESSED_PICS_FOLDER_SIZE: usize = 150;
