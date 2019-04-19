<?php
require 'vendor/autoload.php';

$app = new \Slim\App();

$loader = new Twig_Loader_Filesystem('templates');
$twig = new Twig_Environment($loader);

$app->get('/', function ($request, $response, $args) use ($twig) {
    $data = [];
    $data['price'] = 100;
    $html = $twig->render('tax.html', $data);
    return $response->write($html);
});

$app->post('/', function($request, $response, $args) use ($twig) {
    $data = $request->getParsedBody();
    if ($data['field'] == "include") {
      $data['rate'] = 8;
      $data['taxinclude'] = $data['price'];
      $data['tax'] = $data['price'] / 0.08;
      $data['taxexclude'] = $data['price'] / 1.08;
    } else {
      $data['rate'] = 8;
      $data['taxinclude'] = $data['price'] * 1.08;
      $data['tax'] = $data['price'] * 0.08;
      $data['taxexclude'] = $data['price'];
    }
    $html = $twig->render('tax.html', $data);
    return $response->write($html);
});

$app->run();
