// This module contains logos and methods to work with logos for supported
// blockchains.

use crate::detect::AddressChain;

pub const LOGO_SOLANA_SVG: &str = r#"<svg version="1.1" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 397.7 311.7" style="enable-background:new 0 0 397.7 311.7;" xml:space="preserve">
<title>Solana</title>
<style type="text/css">
.st0{fill:url(#SVGID_1_);}
.st1{fill:url(#SVGID_2_);}
.st2{fill:url(#SVGID_3_);}
</style>
<linearGradient id="SVGID_1_" gradientUnits="userSpaceOnUse" x1="360.8791" y1="351.4553" x2="141.213" y2="-69.2936" gradientTransform="matrix(1 0 0 -1 0 314)">
<stop  offset="0" style="stop-color:#00FFA3"/>
<stop  offset="1" style="stop-color:#DC1FFF"/>
</linearGradient>
<path class="st0" d="M64.6,237.9c2.4-2.4,5.7-3.8,9.2-3.8h317.4c5.8,0,8.7,7,4.6,11.1l-62.7,62.7c-2.4,2.4-5.7,3.8-9.2,3.8H6.5
c-5.8,0-8.7-7-4.6-11.1L64.6,237.9z"/>
<linearGradient id="SVGID_2_" gradientUnits="userSpaceOnUse" x1="264.8291" y1="401.6014" x2="45.163" y2="-19.1475" gradientTransform="matrix(1 0 0 -1 0 314)">
<stop  offset="0" style="stop-color:#00FFA3"/>
<stop  offset="1" style="stop-color:#DC1FFF"/>
</linearGradient>
<path class="st1" d="M64.6,3.8C67.1,1.4,70.4,0,73.8,0h317.4c5.8,0,8.7,7,4.6,11.1l-62.7,62.7c-2.4,2.4-5.7,3.8-9.2,3.8H6.5
c-5.8,0-8.7-7-4.6-11.1L64.6,3.8z"/>
<linearGradient id="SVGID_3_" gradientUnits="userSpaceOnUse" x1="312.5484" y1="376.688" x2="92.8822" y2="-44.061" gradientTransform="matrix(1 0 0 -1 0 314)">
<stop  offset="0" style="stop-color:#00FFA3"/>
<stop  offset="1" style="stop-color:#DC1FFF"/>
</linearGradient>
<path class="st2" d="M333.1,120.1c-2.4-2.4-5.7-3.8-9.2-3.8H6.5c-5.8,0-8.7,7-4.6,11.1l62.7,62.7c2.4,2.4,5.7,3.8,9.2,3.8h317.4
c5.8,0,8.7-7,4.6-11.1L333.1,120.1z"/>
</svg>
"#;

pub const LOGO_ETHEREUM_SVG: &str = r#"<svg role="img" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg">
<title>Ethereum</title>
<path d="M11.944 17.97L4.58 13.62 11.943 24l7.37-10.38-7.372 4.35h.003zM12.056 0L4.69 12.223l7.365 4.354 7.365-4.35L12.056 0z"/>
</svg>
"#;

pub const LOGO_BITCOIN_SVG: &str = r#"<svg role="img" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg">
<title>Bitcoin</title>
<path d="M23.638 14.904c-1.602 6.43-8.113 10.34-14.542 8.736C2.67 22.05-1.244 15.525.362 9.105 1.962 2.67 8.475-1.243 14.9.358c6.43 1.605 10.342 8.115 8.738 14.548v-.002zm-6.35-4.613c.24-1.59-.974-2.45-2.64-3.03l.54-2.153-1.315-.33-.525 2.107c-.345-.087-.705-.167-1.064-.25l.526-2.127-1.32-.33-.54 2.165c-.285-.067-.565-.132-.84-.2l-1.815-.45-.35 1.407s.975.225.955.236c.535.136.63.486.615.766l-1.477 5.92c-.075.166-.24.406-.614.314.015.02-.96-.24-.96-.24l-.66 1.51 1.71.426.93.242-.54 2.19 1.32.327.54-2.17c.36.1.705.19 1.05.273l-.51 2.154 1.32.33.545-2.19c2.24.427 3.93.257 4.64-1.774.57-1.637-.03-2.58-1.217-3.196.854-.193 1.5-.76 1.68-1.93h.01zm-3.01 4.22c-.404 1.64-3.157.75-4.05.53l.72-2.9c.896.23 3.757.67 3.33 2.37zm.41-4.24c-.37 1.49-2.662.735-3.405.55l.654-2.64c.744.18 3.137.524 2.75 2.084v.006z"/>
</svg>
"#;

const LOGO_UNKNOWN_SVG: &str = r#"<svg xmlns="http://www.w3.org/2000/svg">
<title>Unknown</title>
</svg>
"#;

pub fn get_logo(chain: &AddressChain) -> &'static str {
    match chain {
        AddressChain::Bitcoin("P2SH")
        | AddressChain::Bitcoin("P2PKH")
        | AddressChain::Bitcoin("Bech32") => LOGO_BITCOIN_SVG,
        AddressChain::Solana => LOGO_SOLANA_SVG,
        AddressChain::Ethereum => LOGO_ETHEREUM_SVG,
        _ => LOGO_UNKNOWN_SVG,
    }
}

// get_sized_logo adds the given size as the width and height in pixels.
pub fn get_sized_logo(chain: &AddressChain, size_in_pixels: usize) -> String {
    match chain {
        AddressChain::Bitcoin("P2SH")
        | AddressChain::Bitcoin("P2PKH")
        | AddressChain::Bitcoin("Bech32") => get_sized(LOGO_BITCOIN_SVG, size_in_pixels),
        AddressChain::Solana => get_sized(LOGO_SOLANA_SVG, size_in_pixels),
        AddressChain::Ethereum => get_sized(LOGO_ETHEREUM_SVG, size_in_pixels),
        _ => get_sized(LOGO_UNKNOWN_SVG, size_in_pixels),
    }
}

fn get_sized(svg: &str, size_in_pixels: usize) -> String {
    let size = format!(
        r#" width="{:}px" height="{:}px""#,
        size_in_pixels, size_in_pixels
    );
    let index = svg.find(">").unwrap();
    svg[..index].to_string() + &size + &svg[index..]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_sized() {
        let out = get_sized(LOGO_SOLANA_SVG, 256);
        println!("{:?}", out);
    }
}
