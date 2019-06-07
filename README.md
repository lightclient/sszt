# ssz-json

A human readable ssz format using json.

## Format
The format is a subset of JSON and follows most of the conventions closely. The two biggest
deviations are appending the integer type at the end of the number using a string and
declaring the type array (list / vector) in the first element.

| Type | Representation| 
| -- |--| 
| null | `null`
| bool | `true | false`
| number | `"N:{u8|u16|u32|u64|u128|u256}"`
| string | `"string"`
| vector (fixed length) | `["vector", "0:u8", "1:u8"]`
| list (variable length) | `["list", "0:u8", "1:u8"]`
| object | `{ "key1": "123u32", "key2": ["vector", "456u32"] }`

## Examples
```json
{
	"new_messages": [
		"list",
		{
			"timestamp": "1:u64",
			"message": "0:u256"
		},
		{
			"timestamp": "2:u64",
			"message": "454086624460063511464984254936031011189294057512315937409637584344757371137:u256"
		}
	],
	"state": {
		"messages": ["list"]
	}
}
```
ssz encoding: `5c0000005000000001000000000000000000000000000000000000000000000000000000000000000000000000000000020000000000000000010101010101010101010101010101010101010101010101010101010101010400000000000000`

--

```json
{
	"messages": [
		"list",
		{
			"timestamp": "1:u64",
			"message": "0:u256"
		},
		{
			"timestamp": "2:u64",
			"message": "454086624460063511464984254936031011189294057512315937409637584344757371137:u256"
		}

	]
}
```
ssz encoding: `5c0000005000000001000000000000000000000000000000000000000000000000000000000000000000000000000000020000000000000001010101010101010101010101010101010101010101010101010101010101010400000000000000`

--

```json
{
	"fixed": [
		"vector",
		["vector", "0:u8"],
		["vector", "1:u8"],
		["vector", "2:u8"],
		["vector", "3:u8"],
		["vector", "4:u8"]
	],
	"other": {
		"a": {
			"b": {
				"c": {
					"d": [
						"vector",
						"16:u16"
					]
				}
			}
		},
		"b": "32:u32"
	}
}
```
ssz encoding: `0001020304100020000000`

## Maintainer
* [@matt_garnett](https://twitter.com/matt_garnett)

## License
Apache 2.0
