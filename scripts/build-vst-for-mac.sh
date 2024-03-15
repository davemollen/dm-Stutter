name="dm-Reverb"
binary_name="libdm_reverb.dylib"
vst_name="$name.vst"
move_to="/Library/Audio/Plug-Ins/VST/$vst_name"

cd vst
cargo build --release
cd target/release
../../../scripts/osx_vst_bundler.sh $name $binary_name 