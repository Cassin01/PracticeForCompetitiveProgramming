<?php
require 'vendor/autoload.php';

// データベース接続
try {
    $pdo = new PDO('sqlite:vending.db');
} catch (PDOException $e) {
    die("データベース接続失敗" . $e->getMessage());
}
// テーブル作成
$pdo->exec("create table if not exists items (
    id    integer primary key autoincrement,
    name  varchar,
    price integer,
    num   integer)"
);

// SLIM framework の準備
$app = new \Slim\App();

// twig の準備
$loader = new Twig_Loader_Filesystem('templates');
$twig = new Twig_Environment($loader);

//----------------------------------
// ルーティング
//----------------------------------
$app->get('/', function ($request, $response, $args) use ($twig, $pdo) {
    $data = [];
    $data['pay'] = 0;

    $stmt = $pdo->prepare("SELECT * FROM items");
    $stmt->execute();
    $data['items'] = $stmt->fetchALL();

    $html = $twig->render('machine.html', $data);
    return $response->write($html);
});

$app->post('/', function ($request, $response, $args) use ($twig, $pdo) {
    $data = $request->getParsedBody();

    // データ表示テスト
    //echo "<pre>";
    //echo $data["buyitem"][1];
    //echo ">>>>>>" . $data["pay"] . "<br>";
    //var_dump($data);
    //echo "</pre>";
    // データ表示テスト

    $stmt = $pdo->prepare("SELECT * FROM items");
    $stmt->execute();
    $data['items'] = $stmt->fetchALL();


    $data['buyed'] = array();
    $sum = 0;

    for ($i = 0; $i < count($data['buyitem']); $i++) {
        $sum = $sum + $data['buyitem'][$i + 1] * $data['items'][$i]['price'];
    }
    for ($i = 0; $i < count($data['buyitem']); $i++) {
        if ($data['buyitem'][$i + 1] > 0) {
            array_push($data['buyed'], $data['items'][$i]['name']);
        }
    }

    $data['result'] = True;
    $data['sum'] =  $sum;
    $turi = $data['pay'] - $sum;
    $data['turi'] = $turi;

    if ($sum > $data['pay']) {
       $data['lessmoney'] = "投入金額不足";
    }
    else {
      $data['lessmoney'] = "";
      for ($i = 0; $i < count($data['buyitem']); $i++) {
         $lest = $data['items'][$i]['num'] - $data['buyitem'][$i + 1];
         if ($lest > 0) {
             $stmt = $pdo->prepare("update items set num = ? where id = ?");
             $stmt -> execute([$lest, $data['items'][$i]['id']]);
         }
      }
    }


    //var_dump($data['items']);

    $html = $twig->render('machine.html', $data);
    return $response->write($html);
});

$app->get('/insert/{name}/{num}', function ($request, $response, $args) use ($twig, $pdo) {
    $stmt = $pdo->prepare("insert into items (name, price, num) values (?,?,?)");
    $stmt->execute([$args['name'], 0, $args['num']]);

    return $response->write('新商品' . $args['name'] . 'を' . $args['num'] . '個追加しました');
});

//商品削除
$app->get('/delete/{name}', function ($request, $response, $args) use ($twig, $pdo) {
    $stmt = $pdo->prepare("delete from items where name = ?");
    $stmt->execute([$args['name']]);

    return $response->write('商品' . $args['name'] . 'を削除しました');
});


//価格設定
$app->get('/set/{name}/{price}', function ($request, $response, $args) use ($twig, $pdo) {
    $stmt = $pdo->prepare("update items set price = ? where name = ?");
    $stmt->execute([$args['price'], $args['name']]);
    //$stmt->execute([$args['price']], [$args['name']]);

    return $response->write('商品' . $args['name'] . 'の価格を' . $args['price'] . 'に変更しました');
});

//数量追加
$app->get('/add/{name}/{num}', function ($request, $response, $args) use ($twig, $pdo) {
    $stmt = $pdo->prepare("SELECT * FROM items");
    $stmt->execute();
    $data['items'] = $stmt->fetchALL();

    $doflag = 0;
    for ($i = 0; $i < count($data['items']); $i++) {
        if ($data['items'][$i]['name'] == $args['name']) {
            $doflag = 1;    
            $newnum = $data['items'][$i]['num'] + $args['num'];
        }
    }

    if ($doflag == 1) {
        $stmt = $pdo->prepare("update items set num = ? where name = ?");
        $stmt->execute([$newnum, $args['name']]);
        return $response->write('商品' . $args['name'] . 'の個数を' . $newnum . 'に変更しました');
    }
    else {
        return $response->write('商品' . $args['name'] . "は見つかりませんでした");
    }
    //$stmt->execute([$args['price']], [$args['name']]);
});

$app->run();
