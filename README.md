# xwf-api-rs

Unofficial Rust Bindings for **X-Ways Forensics** X-Tension API


## Description

This project provides Rust bindings for the X-Ways Forensics X-Tension API, 
allowing developers to create powerful forensic extensions in Rust.

By leveraging the high performance and safety features of Rust, 
this binding enables seamless integration with the X-Tension API, 
facilitating the development of custom forensic tools and automation solutions 
for digital forensics workflows.


## Example Usage
In _example_ subdirectory reside some sample projects that 
shall demonstrate basic usage of the xwf-api-rs library 
and can be used as a boilerplate for new projects.

A quick guide how to use this library for developing own X-Tensions can be found [here](docs/quick_start.md).

Currently following examples are existing:
- [**xt-helloworld-rs**](examples/xt-helloworld-rs)
  - shows a minimum working example for a X-Tension
  - example for getting user inputs and showing progress bar
  - shows how to define minimum required XWF API Version in cargo.toml
- [**xt-count-items-rs**](examples/xt-count-items-rs)
  - shows how to iterate over evidences and items
  - shows how to get some attributes of an item object
  - shows how to define minimum required XWF API Version in cargo.toml
- [**xt-process-data-rs**](examples/xt-process-data-rs)
  - shows how to use xt_process_item_ex() for calling X-Tensions via _Volume Snapshot Refinement_
  - shows how to get some data portions of an item
  - shows how to get case information and computing a unique id for items
  - shows how to export item data to any location
- [**xt-item-parser-rs**](examples/xt-process-data-rs)
  - shows how to iterate over evidences and items
  - shows how to write some item information to a CSV file
  

## Current state of development
**Version 1.0.0 released.**

Not all available functionality of the X-Tension C API has been translated to Rust yet.

However, the implemented parts are stable and can already be used for certain production use cases, 
provided they meet your requirements.

The architectural concept of the API should be quite stable and settled, 
having been tested and used in some real-world scenarios.

From now on, any incompatible changes to the existing API 
will result in a major version increment. 
The goal is to maintain backward compatibility 
for all upcoming 1.x.x versions in accordance with [semver.org](https://semver.org/).


### Available Features
- macro expressions for defining and exporting all required DLL-functions for X-Tension, 
  so the needed boilerplate-code for new projects reduced to the absolute bare minimum.
- compile-time definition of desired XWF API Level (via rust feature)
  - behaviour *xwf-api-rs* will adapt its functionality according to the specified XWF-API version
  - automatic runtime checks for XWF version to ensure X-Tension compatibility
- convenient macro-expressions for XWF log outputs (xwfinfo!(...), xwfwarn!(...), xwfdebug!(...))
- proper enumeration- and bitflag-types for most of numerical XWF-API values 
- proper error handling
  - runtime error checks for most XWF-API calls
  - well-defined error-type XwfError represents all different kinds of API failures
  - handling of undocumented/newly-introduced enumeration-values from XWF C-API
- object-oriented interface interacting with "items", "evidences" and "volumes"
  - getters and setters for most of available item attributes
  - iterate over parent items
  - read binary data from items
  - create new items from data in memory
  - (de-)serialization capability for most of item-attributes
  - "UniqueItemId"-type for identifying items across different evidences
- multithreading capability
- convenience functions for iterating over all evidences and items
- convenience functions for accessing/querying items within report tables


## Project Goals
- Translating all relevant features of XWF X-Tension API into Rust Language
- Providing an object-oriented and high-level abstraction layer 
  to the official X-Tension C API
- Use idiomatic Rust features to facilitate a safe, robust and intuitive way of using the API
- Simplify project setup for new X-Tensions
- Identify possible problems and bugs within the XWF API functions


## License
This project is licensed under the **LGPLv3 (Lesser General Public License)**. 
This means that while you can freely use and integrate this project with proprietary software, 
any modifications to the binding itself must be released under the same license.
For more details, see the LICENSE file.

Please note that in addition to the LGPLv3 license, the usage of the _X-Ways X-Tension API_ is subject to the licensing terms of _X-Ways_ software. 
Ensure that you comply with the terms and conditions set by _X-Ways Software Technology AG_ for the use of their software and API.

## Disclaimer
This project is not developed by or affiliated with _X-Ways Software Technology AG_ in any way.

This project is provided "as is," without warranty of any kind, express or implied, 
including but not limited to the warranties of merchantability, 
fitness for a particular purpose, and noninfringement. 
In no event shall the author be liable for any claim, damages, or other liability, 
whether in an action of contract, tort, or otherwise, arising from, out of, 
or in connection with the software or the use or other dealings in the software. 
Users are solely responsible for any risks or issues that may arise from the use of this binding, 
including but not limited to data loss, system failures, 
or incorrect forensic analysis results.



## Contribution
Contributions are welcome! If you would like to contribute to this project, 
feel free to open an issue or submit a pull request. 
Please ensure that your contributions adhere to the LGPLv3 license.


