# round-to

> Based on round-to NPM Crate
> Round a number to a specific number of decimal places: `1.234` → `1.2`
> Supports negative numbers as well

## Install

```sh
cargo add round-to
```

## Usage

```js
use round-to::RoundTo;

RoundTo::round_to(1.234, 2);
//=> 1.23

RoundTo::round_to_ceil(1.234, 2);
//=> 1.24

RoundTo::round_to_floor(1.234, 2);
//=> 1.23
```

## API

### number

Type: `number`

The number to adjust.

### precision

Type: `number` *(Integer or infinity)*

The number of decimal places.
