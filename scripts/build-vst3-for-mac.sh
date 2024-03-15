package_name="dm_reverb"
move_from="./target/bundled/$package_name.vst3"
move_to="/Library/Audio/Plug-Ins/VST3/dm-Reverb.vst3"

cd nih-plug
cargo xtask bundle $package_name --release
mv ./target/release/$package_name ./target/release/dm-Reverb.vst3