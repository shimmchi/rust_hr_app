# 概要
- 人が部署に所属することを管理するためのアプリケーション
- コマンドライン上でのテキストインターフェースで操作する
- 操作結果はテキストファイルに保存され、次回実行時には前回の内容を読み込む

## 可能な操作

 - 人の作成
 - 人の削除
 - 部署の作成
 - 部署の削除
 - 人を部署に所属させる
 - 人を部署から外す
 - 各種情報の表示

## 実装について

オブジェクト指向を意識した。

また、ドメイン駆動設計やヘキサゴーナルアーキテクチャといった概念を参考にしている。

### ヘキサゴーナルアーキテクチャの参考サイト

[GoではじめるHexagonal Architecture](https://qiita.com/usk81/items/5fd99c1c84d9a15db583)

[Hexagonal Architecture](https://nrslib.com/hexagonal-architecture/)

[Example Implementation of a Hexagonal Architecture](https://github.com/thombergs/buckpal)

# 使用方法

## アプリケーション起動
```
$ cargo run
```

## 人を作成
```
> create_person <last_name> <first_name>
```
first_nameは省略可能
## 作成した人の一覧を表示
```
> show_all_persons
```
- 自動で割り振られるIDと人の名前が一覧で表示される
- ここで表示されるIDを使用して以後の人の操作を行う
  
## 人を削除する
```
> delete_person <person_id>
```
- ここで指定するのはIDなので注意
- 人の名前で個人を指定することは出来ない(同名の人が複数存在する可能性があるため)
  
## 部署の作成, 表示, 削除
```
> create_department <department_name>
> show_all_departments
> delete_department
```
- 部署を削除しても所属している人は削除されない

## 人を部署に所属させる, 外す
```
> add_person <person_id> <department_id>
> remove_person <person_id> <department_id>
```

## 人を指定して所属部署の一覧を表示する
```
> person_info <person_id>
```

## 部署を指定して所属している人の一覧を表示する
```
> department_info <department_id>
```

## 全ての情報を表示する
```
> all_info
```
- 全ての部署について、その部署に所属している人の名前が一覧表示される
- どの部署にも所属していない人は最後にまとめて表示される

## Help
```
> help
```
コマンドのヘルプが表示される

## アプリケーションを終了する
```
> quit
```
