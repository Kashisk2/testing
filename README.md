# WUW (Whats Ur Wallet) [![Build](https://github.com/euforia/wuw/actions/workflows/rust.yml/badge.svg)](https://github.com/euforia/wuw/actions/workflows/rust.yml)

WUW is a service that attempts to identify the blockchain associated to a given wallet address.  This
service returns an SVG of the blockchain logo for the input wallet address. 

It is meant to be a very simple and super lightweight service.

## Usage 

### API

Identifiy the blockchain of the public key and get the SVG logo

```
GET /{base58_encoded_public_key}[?size={svg_logo_diameter}]
```

Identify the blockchain of the public key and return a JSON object containing the name and the SVG logo string.

```
GET /{base58_encoded_public_key}[?size={svg_logo_diameter}]
Accept: application/json
```

