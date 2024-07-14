## [1.0.85](https://github.com/zhangyuang/node-ffi-rs/compare/v1.0.84...v1.0.85) (2024-07-14)


### Features

* add tips for jsobject/array when used in new thread ([41da725](https://github.com/zhangyuang/node-ffi-rs/commit/41da725e600365fcc59788cf98d636767d87cbcb))



## [1.0.84](https://github.com/zhangyuang/node-ffi-rs/compare/v1.0.83...v1.0.84) (2024-07-14)


### Features

* support pass jsFunction return value to c ([#60](https://github.com/zhangyuang/node-ffi-rs/issues/60)) ([44c19be](https://github.com/zhangyuang/node-ffi-rs/commit/44c19be94234a6dc5724d08cf379b76fc8daa25f))



## [1.0.83](https://github.com/zhangyuang/node-ffi-rs/compare/v1.0.82...v1.0.83) (2024-06-20)


### Bug Fixes

* return bigInt when use struct field ([0be42a7](https://github.com/zhangyuang/node-ffi-rs/commit/0be42a78005449f9f40783e73779ca8e85dfcb1b))


### Features

* update d.ts ([7aeeb20](https://github.com/zhangyuang/node-ffi-rs/commit/7aeeb20b63815627d17a9b87e2e104d14e483826))



## [1.0.82](https://github.com/zhangyuang/node-ffi-rs/compare/v1.0.81...v1.0.82) (2024-06-15)


### Bug Fixes

* No need to free jsBuffer memory in rust call params ([2e39791](https://github.com/zhangyuang/node-ffi-rs/commit/2e39791948a7634ec5dff493df5915bf575a9414))



## [1.0.81](https://github.com/zhangyuang/node-ffi-rs/compare/v1.0.80...v1.0.81) (2024-06-14)


### Features

* support DataType.BigInt ([cf05fab](https://github.com/zhangyuang/node-ffi-rs/commit/cf05fab803527174d29e96371d3a0a3bf9d55389))



## [1.0.80](https://github.com/zhangyuang/node-ffi-rs/compare/v1.0.79...v1.0.80) (2024-06-13)


### Features

* set freeResultMemory as false at default ([ddcf108](https://github.com/zhangyuang/node-ffi-rs/commit/ddcf108a0b9d2beb4c9ffc3ba928a78429bf3e20))



## [1.0.79](https://github.com/zhangyuang/node-ffi-rs/compare/v1.0.78...v1.0.79) (2024-06-11)


### Features

* support wideString ([a9828b2](https://github.com/zhangyuang/node-ffi-rs/commit/a9828b2423d867b7628a3d9b9e2c82fa485bf9de))
* use ref as params type avoid clone ([b9d37fb](https://github.com/zhangyuang/node-ffi-rs/commit/b9d37fb1875a84a26ffc59c1be77bd776cface4b))
* use trait to implement feature insteadof separate function ([f2679b9](https://github.com/zhangyuang/node-ffi-rs/commit/f2679b90880dd7b44d3f802b4d1b6ab60e04ab38))



## [1.0.78](https://github.com/zhangyuang/node-ffi-rs/compare/v1.0.77...v1.0.78) (2024-05-29)


### Features

* support stack struct ([#41](https://github.com/zhangyuang/node-ffi-rs/issues/41)) ([28cb2d4](https://github.com/zhangyuang/node-ffi-rs/commit/28cb2d492bf3b638b4301347f4878da94a4ddcc3))



## [1.0.77](https://github.com/zhangyuang/node-ffi-rs/compare/v1.0.77-alpha.1...v1.0.77) (2024-05-21)



## [1.0.77-alpha.1](https://github.com/zhangyuang/node-ffi-rs/compare/v1.0.77-alpha.0...v1.0.77-alpha.1) (2024-05-20)


### Features

* add freeCFuncParamsMemory in funcConstructor ([c5f298e](https://github.com/zhangyuang/node-ffi-rs/commit/c5f298ed12856954fa6ede8ea8cbd30aa328ae81))



## [1.0.77-alpha.0](https://github.com/zhangyuang/node-ffi-rs/compare/v1.0.76...v1.0.77-alpha.0) (2024-05-20)


### Bug Fixes

* add freePointer, free memory after ffi-call prevent memory leak ([#40](https://github.com/zhangyuang/node-ffi-rs/issues/40)) ([f639628](https://github.com/zhangyuang/node-ffi-rs/commit/f639628ce8297a64aa72bc14ff00aaf5fcc2a147))



## [1.0.76](https://github.com/zhangyuang/node-ffi-rs/compare/v1.0.75...v1.0.76) (2024-05-16)


### Features

* add issue template ([e4aef11](https://github.com/zhangyuang/node-ffi-rs/commit/e4aef11ad3c6aa3924de7dfe92b0be765a842483))
* update OpeningLibraryError tips ([0aa86b4](https://github.com/zhangyuang/node-ffi-rs/commit/0aa86b4d682edb565b1233381b6b14a93ee9abb4))



## [1.0.75](https://github.com/zhangyuang/node-ffi-rs/compare/v1.0.74...v1.0.75) (2024-05-10)


### Features

* add aarch64-pc-windows-msvc artifact ([23b3118](https://github.com/zhangyuang/node-ffi-rs/commit/23b3118da0bdbf93d094fc6daf22a5a4451d032b))
* update publish script support alpha ([45ca8e3](https://github.com/zhangyuang/node-ffi-rs/commit/45ca8e3e633ae9dc3e24169ad950483dc1953b64))



## [1.0.74](https://github.com/zhangyuang/node-ffi-rs/compare/v1.0.73...v1.0.74) (2024-05-07)


### Features

* check whether params_type length and params_value length is equal ([dd9260c](https://github.com/zhangyuang/node-ffi-rs/commit/dd9260c9e695a3dd349a4046c1b8b892214ae8bf))



## [1.0.73](https://github.com/zhangyuang/node-ffi-rs/compare/v1.0.73-0...v1.0.73) (2024-05-07)


### Features

* modifying all string handling functions to uniform UTF-16 handling ([19573b4](https://github.com/zhangyuang/node-ffi-rs/commit/19573b442e88069fec0855a77d1c920a3b079f51))



## [1.0.73-0](https://github.com/zhangyuang/node-ffi-rs/compare/v1.0.72...v1.0.73-0) (2024-05-07)


### Bug Fixes

* convert jsString to UTF-16 string, create cstring from bytes manually ([5fad2d0](https://github.com/zhangyuang/node-ffi-rs/commit/5fad2d043b3457aa183bbeba72fe4b0350f8d2d6))



## [1.0.72](https://github.com/zhangyuang/node-ffi-rs/compare/v1.0.69...v1.0.72) (2024-05-06)


### Bug Fixes

* convert JsString to UTF-16 string ([70ea15c](https://github.com/zhangyuang/node-ffi-rs/commit/70ea15cac7899c90628c31ee67f05125bed094cf))
* convert JsString to UTF-8 string ([c613f66](https://github.com/zhangyuang/node-ffi-rs/commit/c613f666ce5651d2f358e21294aebf01d1c2bfe6))


### Features

* dynamicArray support more array type ([170e9fb](https://github.com/zhangyuang/node-ffi-rs/commit/170e9fb7a17a4cfc0ef6bc39a9e49327bdc403f7))
* suppprt freePointer ([dbd50bb](https://github.com/zhangyuang/node-ffi-rs/commit/dbd50bbb96ac4415c759d925c6dc50fa7f8fe9d3))



## [1.0.69](https://github.com/zhangyuang/node-ffi-rs/compare/v1.0.64...v1.0.69) (2024-04-30)


### Bug Fixes

* ci add darwin-x64 target ([55906a7](https://github.com/zhangyuang/node-ffi-rs/commit/55906a778e4fe1849181356083841962cc4e331b))


### Features

* add arrayConstructor judge ([bebcc94](https://github.com/zhangyuang/node-ffi-rs/commit/bebcc94352da2d4bf15098fa6d8bc16b1bf2a9bb))
* add changelog ([403333c](https://github.com/zhangyuang/node-ffi-rs/commit/403333cad4cf9e4242a810b8d9c9df72fabd2f7e))
* implement runInNewThread ([#31](https://github.com/zhangyuang/node-ffi-rs/issues/31)) ([f908400](https://github.com/zhangyuang/node-ffi-rs/commit/f9084008e5484a5c2bf25abca7a940f9cf99480d))
* judge library path exist or not before call open ([a97c26f](https://github.com/zhangyuang/node-ffi-rs/commit/a97c26f978d37d68bd22a88e89d05e12817ac4c4))
* mock jsExternal type ([479daab](https://github.com/zhangyuang/node-ffi-rs/commit/479daabc44d87f667f97be97c51adbc16f31f618))
* refactor get_js_external_wrap_data ([9f6878c](https://github.com/zhangyuang/node-ffi-rs/commit/9f6878cf16fd88308275b572745047a83d0b8061))
* use unchecked napi transform improve performance ([8100d8a](https://github.com/zhangyuang/node-ffi-rs/commit/8100d8a545b0351e013b55d9ccc2f19866e4ed67))



## [1.0.64](https://github.com/zhangyuang/node-ffi-rs/compare/v1.0.63...v1.0.64) (2024-04-19)


### Features

* support DataType.float ([0f2ec4e](https://github.com/zhangyuang/node-ffi-rs/commit/0f2ec4e2ef6c4bbdb5b55343a6509bde53820322))



## [1.0.63](https://github.com/zhangyuang/node-ffi-rs/compare/v1.0.61...v1.0.63) (2024-04-18)


### Features

* add define methods to define function signature ([5945682](https://github.com/zhangyuang/node-ffi-rs/commit/59456824acddcdb52a5d1bae1c593c5cdf44b170))
* refactor type hint ([dcda3a8](https://github.com/zhangyuang/node-ffi-rs/commit/dcda3a8064422cbb46fa8993d764b4f805950cd1))
* support floatArray as paramsType ([a7fef73](https://github.com/zhangyuang/node-ffi-rs/commit/a7fef73d1b753d914cf6a64145a76ad6eaa592ee))



## [1.0.61](https://github.com/zhangyuang/node-ffi-rs/compare/v1.0.60...v1.0.61) (2024-04-14)


### Features

* add ResultWithErrno type support ([c21c375](https://github.com/zhangyuang/node-ffi-rs/commit/c21c375094927aa03e39d33e70001393eaec42be))



## [1.0.60](https://github.com/zhangyuang/node-ffi-rs/compare/v1.0.59...v1.0.60) (2024-04-14)


### Features

* support output errno info ([ae265fb](https://github.com/zhangyuang/node-ffi-rs/commit/ae265fbec4554b4aff1c5d84366aa89e3c05a7be))
* update error tips when open uncompatible share library ([a947241](https://github.com/zhangyuang/node-ffi-rs/commit/a947241f7c0ac33e01f25c566ea84cc2b3baa0ae))



## [1.0.59](https://github.com/zhangyuang/node-ffi-rs/compare/v1.0.58...v1.0.59) (2024-04-13)


### Features

* support static array in c struct field ([890a310](https://github.com/zhangyuang/node-ffi-rs/commit/890a310159e46c84c9d1d0df40ef7689f7de3e83))



## [1.0.58](https://github.com/zhangyuang/node-ffi-rs/compare/v1.0.57...v1.0.58) (2024-04-09)


### Features

* implement create clousre by libffi::middle support more types ([#25](https://github.com/zhangyuang/node-ffi-rs/issues/25)) ([608237f](https://github.com/zhangyuang/node-ffi-rs/commit/608237f98415929b4672405cb809f7bbcb01f7a1))



## [1.0.57](https://github.com/zhangyuang/node-ffi-rs/compare/v1.0.55...v1.0.57) (2024-04-08)


### Bug Fixes

* publish main package ([66ef423](https://github.com/zhangyuang/node-ffi-rs/commit/66ef423bf829f723179302ac8718b358526d0a7e))


### Features

* rename unpackPointer to unwrapPointer ([8259780](https://github.com/zhangyuang/node-ffi-rs/commit/825978090e25835f6b1f4b24396ce040a0f2cf4f))



## [1.0.55](https://github.com/zhangyuang/node-ffi-rs/compare/v1.0.54...v1.0.55) (2024-04-07)


### Features

* support wrapPointer and unpackPointer ([4406477](https://github.com/zhangyuang/node-ffi-rs/commit/4406477f99ba5e4151d6d000802bd1b4cf22b1f2))



## [1.0.54](https://github.com/zhangyuang/node-ffi-rs/compare/v1.0.53...v1.0.54) (2024-04-06)


### Bug Fixes

* createPointer for struct ([cf6c237](https://github.com/zhangyuang/node-ffi-rs/commit/cf6c237aeeead0d21fe23901034356aaa0501a60))
* update createPointer logic ([9b61dc6](https://github.com/zhangyuang/node-ffi-rs/commit/9b61dc645e07d56bf69c30a2fe4638f812df1142))


### Features

* cache function symbol in global variables ([b08f157](https://github.com/zhangyuang/node-ffi-rs/commit/b08f1575a589d41fd2b86082e8f5cb1cd2acec74))



## [1.0.53](https://github.com/zhangyuang/node-ffi-rs/compare/v1.0.52...v1.0.53) (2024-04-05)


### Bug Fixes

* add libnative only in linux/darwin ([89ac7e4](https://github.com/zhangyuang/node-ffi-rs/commit/89ac7e4ccded66745b95bd30308154e9984c8da3))


### Features

* support linux-x64-musl ([02a1b43](https://github.com/zhangyuang/node-ffi-rs/commit/02a1b4300c6eb89d4937b2c3cf2a47b5c5c73f59))
* support Load Main Program handle ([8e821b8](https://github.com/zhangyuang/node-ffi-rs/commit/8e821b8145f88a803e5836af12d34f13575fc052))



## [1.0.52](https://github.com/zhangyuang/node-ffi-rs/compare/39596d5e0aadf78912695fd2b98b825699ad1767...v1.0.52) (2024-04-04)


### Bug Fixes

* avoid xlocale/_stdio.h outside (macOS only) ([4caaaa6](https://github.com/zhangyuang/node-ffi-rs/commit/4caaaa6f654198f550ba676806b0f7b49902310c))
* calculate struct pointer offset padding ([9ee7c10](https://github.com/zhangyuang/node-ffi-rs/commit/9ee7c10542d9921d85c1468406e90be842a71a7b))
* ci ([69214d0](https://github.com/zhangyuang/node-ffi-rs/commit/69214d0a35ecb7f45b07afb1ae233f60dd5af464))
* dependencies ([ef4de0d](https://github.com/zhangyuang/node-ffi-rs/commit/ef4de0d9d048923fc267c96a028097771fe02b77))
* object pointer offset position ([df1d0c3](https://github.com/zhangyuang/node-ffi-rs/commit/df1d0c3201cccc9c98aef56a3ca98af84769e3f9))
* return buffer ([026141c](https://github.com/zhangyuang/node-ffi-rs/commit/026141cc1276fc7a5cb6e2e3d4588abf37c37145))
* struct offset position ([74d7587](https://github.com/zhangyuang/node-ffi-rs/commit/74d7587a398ff7c563797aece89ca0a236ce6a8b))
* use cstr create string for protect ownership ([562bfc6](https://github.com/zhangyuang/node-ffi-rs/commit/562bfc65fadbb793ceafae53864bbd97d9bbea86))
* use cstr return string ([33e0d4c](https://github.com/zhangyuang/node-ffi-rs/commit/33e0d4c95ab7a85df1ac558f88a6a5ed2cb5b266))
* use explicit path to load libsum.so ([21e2b23](https://github.com/zhangyuang/node-ffi-rs/commit/21e2b23d36ebc560e4dd81d8e68556d7ce596d9b))
* use koffi correctly for accurate benchmark ([e0ecfc5](https://github.com/zhangyuang/node-ffi-rs/commit/e0ecfc523478cbca43a8c6bdc36775664049854b))
* win32 ([19fa8bc](https://github.com/zhangyuang/node-ffi-rs/commit/19fa8bc3b41bfd5bcfc6b3b72ebc9f7e0f581e1a))


### Features

* add basictype and reftype ([79aa944](https://github.com/zhangyuang/node-ffi-rs/commit/79aa944f731ae9357e61dec2c7212aa75487686c))
* add benchmark ([e6a0b5e](https://github.com/zhangyuang/node-ffi-rs/commit/e6a0b5e30660e64877387aff4aad4ab0a4ce8d1d))
* add benchmark ([5c9b1cc](https://github.com/zhangyuang/node-ffi-rs/commit/5c9b1cc9b6b80ff70b407811900d81049fc8825c))
* add call c++ class example ([be1d8ac](https://github.com/zhangyuang/node-ffi-rs/commit/be1d8acb5cbaab17b13520d880c4c6ac9fc25728))
* add DataType.void ([1d0ff24](https://github.com/zhangyuang/node-ffi-rs/commit/1d0ff248bcc4a098cf61c060c40d7afd1bd5e6ea))
* add ffi-rs ([79e9594](https://github.com/zhangyuang/node-ffi-rs/commit/79e95940d31c1e7852a892c123229fe8bdd3010c))
* add ffi-rs ([39596d5](https://github.com/zhangyuang/node-ffi-rs/commit/39596d5e0aadf78912695fd2b98b825699ad1767))
* add FunctionNotFound err type ([fb430b9](https://github.com/zhangyuang/node-ffi-rs/commit/fb430b9e2bd032380c8da90761e7f14c6861ccf8))
* add get_data_type_size_align ([10487ed](https://github.com/zhangyuang/node-ffi-rs/commit/10487ed5c3d4c8a112150db8812bcab93d7bdc4b))
* add get_js_unknown_from_pointer ([8a08f39](https://github.com/zhangyuang/node-ffi-rs/commit/8a08f39cd667ad61c0332492c1721dee0b9c6057))
* add github Dependabot ([#16](https://github.com/zhangyuang/node-ffi-rs/issues/16)) ([eb41e0d](https://github.com/zhangyuang/node-ffi-rs/commit/eb41e0d61055c14595d4cf4e3e74a48fd96465ce))
* add scope ([c368e43](https://github.com/zhangyuang/node-ffi-rs/commit/c368e433a6daeb5e03a79b27bcca0fa5db34bda9))
* add ToJsArray trait ([2e1cebc](https://github.com/zhangyuang/node-ffi-rs/commit/2e1cebcb6f532fd3c62dc86dc2a6494405296bef))
* add ToRsArray trait ([c7423ce](https://github.com/zhangyuang/node-ffi-rs/commit/c7423ce6fac7a1611fdfa58de21563cf7cb524b4))
* call tsfn in c other thread ([7c8b0dc](https://github.com/zhangyuang/node-ffi-rs/commit/7c8b0dccc80d2af0487c7c2ca0ba304a83808ff3))
* demo add createPerson ([9456d6a](https://github.com/zhangyuang/node-ffi-rs/commit/9456d6a45b89725e76dfdfd954f92821e4420749))
* eradicate panic ([d1b29c2](https://github.com/zhangyuang/node-ffi-rs/commit/d1b29c296c3d7a95e16484e372c2ad3a2d13c07c))
* function parameter support object ([64bf19e](https://github.com/zhangyuang/node-ffi-rs/commit/64bf19ec6158507cb1fd37a8633cc97923e5a6b8))
* improve threadsafefunction ([9ea02e0](https://github.com/zhangyuang/node-ffi-rs/commit/9ea02e00c4f6abad9678cd1e1acc7e12fb2dba10))
* init ([73856dd](https://github.com/zhangyuang/node-ffi-rs/commit/73856dd12305999dfe0431d97cfa05b28f3e658d))
* optimize ffi_call return value ([80b6985](https://github.com/zhangyuang/node-ffi-rs/commit/80b6985b0f992c2c048336cddac430babce1d9f5))
* rename createExternal to createPointer ([2524c7d](https://github.com/zhangyuang/node-ffi-rs/commit/2524c7d301039d764b34a15c26dc2ae0b4b539d8))
* support boolean type ([f4f7ee9](https://github.com/zhangyuang/node-ffi-rs/commit/f4f7ee98f1dfcffa3ea7bb59158fc8c09b572c7a))
* support byteArray aka u8Array ([341b37c](https://github.com/zhangyuang/node-ffi-rs/commit/341b37c839a0987b784ca7fb3c36056961975244))
* support char type aka u8 ([5c98b30](https://github.com/zhangyuang/node-ffi-rs/commit/5c98b309aa80624b2d1305d1df4cf8d52e23bd2d))
* support createExternal ([8b7a2cb](https://github.com/zhangyuang/node-ffi-rs/commit/8b7a2cb3142c5aa052efe5daeaeda85dc18edc76))
* support double type ([c975817](https://github.com/zhangyuang/node-ffi-rs/commit/c975817a656c9b6c2f2e793ee4678b8e1204410f))
* support doubleArray ([9cdb45e](https://github.com/zhangyuang/node-ffi-rs/commit/9cdb45edb3555d934dd052367240a090134a6317))
* support function as params skip ci ([b75df3d](https://github.com/zhangyuang/node-ffi-rs/commit/b75df3d8b6d028b2b443e1b6bb66bef29ea02069))
* support generate nested object ([bfd0ba3](https://github.com/zhangyuang/node-ffi-rs/commit/bfd0ba32b8d2802a10630ee5990d0ea277328aa8))
* support i32array ([8078d1b](https://github.com/zhangyuang/node-ffi-rs/commit/8078d1bb7ad644f76d0b404090cf972957724138))
* support install x86 artifact on x64 ([2b12695](https://github.com/zhangyuang/node-ffi-rs/commit/2b126953640c967c65c12f211b130fc16adb77ab))
* support long type aka i64 ([6147657](https://github.com/zhangyuang/node-ffi-rs/commit/61476579ebcc47537b69a203740172cd07b02b85))
* support object type ([79e90a8](https://github.com/zhangyuang/node-ffi-rs/commit/79e90a89bf5f2437fb84f784f9a80109b5c38549))
* support open and close dynamic library ([d933720](https://github.com/zhangyuang/node-ffi-rs/commit/d933720189dacd637001f133c66610c929a4fa35))
* support pass js function to ffi-rs ([20741ec](https://github.com/zhangyuang/node-ffi-rs/commit/20741ec298de014e8feaf673794db28475ad13b4))
* support pointer type aka jsexternal ([e091fff](https://github.com/zhangyuang/node-ffi-rs/commit/e091ffffd2fb78582d8a1a621e6e629b4af2c6e0))
* support replace in place ([921fbab](https://github.com/zhangyuang/node-ffi-rs/commit/921fbabc7d2c800ecf741597ed3a7a1ff40816d6))
* support restoreExternal ([b2dd20d](https://github.com/zhangyuang/node-ffi-rs/commit/b2dd20df4b6c0b78f0fc3a13fdfde84da6d7afa7))
* support string array ([2388993](https://github.com/zhangyuang/node-ffi-rs/commit/23889936761f8fe78d50e015b73bc0da7afb2933))
* support struct double type field ([27e40ee](https://github.com/zhangyuang/node-ffi-rs/commit/27e40eeab5126cf4dae1a45b02700326be1cbb83))
* support struct field type doubleArray ([75af21e](https://github.com/zhangyuang/node-ffi-rs/commit/75af21ebc1aefe5431bfc3c0b9adc213ab345c7a))
* support struct field type i32Array ([a2c1274](https://github.com/zhangyuang/node-ffi-rs/commit/a2c1274a9c4f9c14068e34bd7abdb28fa7488b9f))
* support struct with field type stringArr doubleArr ([1c450fb](https://github.com/zhangyuang/node-ffi-rs/commit/1c450fb759cf6a4624b6260789261362bab9d468))
* support u64 fix i64 ([9d10531](https://github.com/zhangyuang/node-ffi-rs/commit/9d105318c2617cd7ab64f473b0bd046e6a73b59f))
* support win32 ([7aa7630](https://github.com/zhangyuang/node-ffi-rs/commit/7aa76302e43826deedd350e7570c14c08549cfaa))
* support x86_64-unknown-linux-musl ([1db4e0b](https://github.com/zhangyuang/node-ffi-rs/commit/1db4e0befc8b538abab7d1770e942bd4f8b325a8))
* thread function ([#12](https://github.com/zhangyuang/node-ffi-rs/issues/12)) ([a17910a](https://github.com/zhangyuang/node-ffi-rs/commit/a17910a9dfe0251a50c1a14ec34068f294d45b6b))
* thread safe function in multiple thread ([94d4af5](https://github.com/zhangyuang/node-ffi-rs/commit/94d4af5387e0a0fe97d122af3de47c798ee86eaa))
* update calculate ([68776da](https://github.com/zhangyuang/node-ffi-rs/commit/68776da09222236968228a95e9e93530d696b78a))
* update d.ts tips ([85c9b03](https://github.com/zhangyuang/node-ffi-rs/commit/85c9b0329d7a89faf88e0c2ce84dffc5ceb39dc6))
* update get_value_pointer method by ? operator ([088e02f](https://github.com/zhangyuang/node-ffi-rs/commit/088e02fb9ee9c99cbee01515d4305ae9e1bf3bf2))
* update optionalDependencies ([30dcfd0](https://github.com/zhangyuang/node-ffi-rs/commit/30dcfd0cbe40382ecc9cc245173190a9f6bc9349))
* update rs_array_to_js_array method by ? operator ([ef4ba5f](https://github.com/zhangyuang/node-ffi-rs/commit/ef4ba5fadd9acd5f92f72e9f8dac8a89c2f7bd49))
* update type_define_to_rs_args method by ? operator ([fe77239](https://github.com/zhangyuang/node-ffi-rs/commit/fe77239cd69c0c55b89872e5bfa4fd7a13ea7589))
* update types ([bd12e4c](https://github.com/zhangyuang/node-ffi-rs/commit/bd12e4ce57b681de3bf7c3ad45cdcdcde0cabb69))
* use ? operator replace unwrap in get_params_value_rs_struct method ([25390e8](https://github.com/zhangyuang/node-ffi-rs/commit/25390e8c6e4f2a9f13a7f5244d6e2ce337a48e75))
* use ? replace unwrap refactor get_arg_types_values method ([2314106](https://github.com/zhangyuang/node-ffi-rs/commit/2314106c1519808f3e5a505950245b97b7d665fa))
* use closure replace closureonce for multicall ([85f255c](https://github.com/zhangyuang/node-ffi-rs/commit/85f255ce80effa7f9b6b0dc1860a30c34f4f0212))
* use cstring replace cstr restore c string from pointer ([704d878](https://github.com/zhangyuang/node-ffi-rs/commit/704d878f9b761c4712375ceec289ac76c2a87d27))
* use dataType replace paramsType ([bb59141](https://github.com/zhangyuang/node-ffi-rs/commit/bb59141948d833bea5bbf716543e358a3465f650))
* use jsnumber as return type ([4108536](https://github.com/zhangyuang/node-ffi-rs/commit/41085368ddeeb4febbbb5e466687a02a589e852b))
* use result style as return type ([9374f78](https://github.com/zhangyuang/node-ffi-rs/commit/9374f78c87ad1c89a2e47e576996574085e30951))
* use safe u8Array when call threadsafefunction ([efb4c06](https://github.com/zhangyuang/node-ffi-rs/commit/efb4c06ef8111b05a934615824de30b94a4bd6ef))



