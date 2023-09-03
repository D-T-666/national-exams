# დესკალიზატორი

```
❯ descale-and-sort -h
გადააქციე ჩარიცხვებისა და რანჟირებული ქულების PDF ფაილი დესკალირებული და დახარისხებული სიად

Usage: descale-and-sort [OPTIONS] <INPUT_FILE> <DESCALING_DATA_FILE> [WORK_PATH]

Arguments:
  <INPUT_FILE>           ჩარიცხვების PDF ფაილი
  <DESCALING_DATA_FILE>  დესკალირების მონაცემების CSV ფაილი
  [WORK_PATH]            დროებითი ფაილების საქაღალდე

Options:
  -g, --graphs         შეიცავდეს გრაფიკებს
  -t, --top-list       შეიცავდეს საკონკურსო ქულის მიხედვით დახარისხებულ სიას
  -f, --faculties      შეიცავდეს ფაკულტეტებს
  -s, --shorten-names  შეამოკლოს უნივერსიტეტების სახელები
  -h, --help           Print help
  -V, --version        Print version
```

#### requirements:

- working instalation of [LaTeX](https://www.latex-project.org/)
- [tabula-py](https://pypi.org/project/tabula-py/)