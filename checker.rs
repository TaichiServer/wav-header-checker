fn is_wav_header_valid(sound_data: &[u8]) -> bool {
  if sound_data.len() < 44 {
      // データが44バイト未満の場合は無効なヘッダーと見なす
      return false;
  }

  let riff: [u8; 4] = [b'R', b'I', b'F', b'F'];
  let exp_header: [u8; 32] = [
      b'W', b'A', b'V', b'E',  // "WAVE"
      b'f', b'm', b't', b' ',  // "fmt "
      16, 0, 0, 0,              // フォーマットデータの長さ
      1, 0,                    // フォーマットタイプ (PCM)
      2, 0,                    // チャンネル数 (ステレオ)
      0x80, 0xBB, 0, 0,        // サンプルレート (48000)
      0x00, 0xEE, 2, 0,        // バイトレート (192000)
      4, 0,                    // ブロックサイズ (4)
      16, 0,                   // サンプルあたりのビット数 (16)
      b'd', b'a', b't', b'a'  // "data"
  ];

  &sound_data[0..4] == riff && &sound_data[8..40] == exp_header
}
