# instrumentRs-drivers

This repository is here to provide instrument drivers
that are based on [`instrumentRs`].
The repository is divided into folders:
One folder per manufacturer at the root.
These manufacturer folders then contain subfolders
for individual instruments,
each of which representing their own crate.
The repo is not set up as a workspace.

## Note

Currently, [`instrumentRs`] has several running drivers in its own repository.
This repository is work under development and will get populated
once [`instrumentRs v0.2.0`] is out.

For updates on [`instrumentRs v0.2.0`], see the repo
as well as the following blog posts:

- [instrumentRs v0.2.0 design ideas]
- [Testing drivers in instrumentRs v0.2.0]

Any drivers currently in here are under development
and are not intended for production use.
They are being developed for testing [`instrumentRs v0.2.0`].

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
  <http://www.apache.org/licenses/LICENSE-2.0>)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.

[`instrumentRs`]: https://github.com/trappitsch/instrumentRs
[`instrumentRs v0.2.0`]: https://github.com/trappitsch/instrumentRs2
[instrumentRs v0.2.0 design ideas]: https://blog.galactic-forensics.space/blog/07-instrumentrs2/
[Testing drivers in instrumentRs v0.2.0] <https://blog.galactic-forensics.space/blog/08-instrumentrs2-mock-interface/>
