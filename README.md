# Yet another [Dreamberd](https://github.com/TodePond/DreamBerd) interpreter




## Roadmap

- [x] Simple lexer
- [ ] AST from lexer output
- [ ] Runner ?

## How to use
```console
cargo r <path/to/my/dreamberd/script.db
```

You'll find a lot of test cases in the `sources` directory


## Testing

### Uses [cargo insta](https://github.com/mitsuhiko/insta) for snapshot testing
```console
sh scripts/test.sh
```
