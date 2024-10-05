# xwf-api-rs

Unofficial Rust Bindings for X-Ways Forensics X-Tension API


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
  

## Current state of development

A detailed API documentation and some more advanced examples are still missing but will follow soon.

Implementation is still very incomplete in respect to the translated features from the X-Tension C API. 
So current functionality may not fit your needs yet.
Furthermore the high-level rust API may still be heavily changed, so I do not recommend to use it productively
unless you are not scared of extensive refactoring sessions.

But besides that, the functionality that is currently implemented seems to work quite well already.
You can easily interact with evidence-objects and their items,
query its attributes such as name, timestamps, paths and parent items, file types/categories, metadata and much more.
It is also possible already to read the binary data of an item.


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


