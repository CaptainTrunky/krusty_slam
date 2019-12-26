use std::path::PathBuf;
use std::fs::read_dir;

pub struct DiskStream {
  pub src_path: PathBuf,
  pub images: Vec<PathBuf>,
  pub depths: Vec<PathBuf>,
}

pub fn build_disk_stream(src_path: PathBuf) -> DiskStream {
  check_dir_path(&src_path);

  let images_path = src_path.join("rgb");
  check_dir_path(&images_path);

  let depth_path = src_path.join("depth");
  check_dir_path(&depth_path);

  let mut images: Vec<PathBuf> = read_dir(images_path).unwrap()
      .map(|p| p.unwrap().path())
      .collect();

  images.sort_by(|a, b| a.file_name().cmp(&b.file_name()));

  let depths = images.iter()
      .map(|p| PathBuf::from(p.to_str().unwrap().replace("rgb", "depth")))
      .collect();

  DiskStream {
    src_path: src_path,
    images: images,
    depths: depths
  }
}

fn check_dir_path(p: &PathBuf) {
  if !p.is_dir() {
    panic!(format!("{} is not available", p.to_str().unwrap()));
  }
}
