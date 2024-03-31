echo "記事一覧の取得"
echo "GET /api/articles"
curl -X GET "http://localhost:8080/api/articles"
echo

sleep 1

echo "新規記事の送信"
echo "POST /api/articles"
# ToDo: subcategory_idは今後複数設定できるようにする
curl -X POST "http://localhost:8080/api/articles" \
     -H "Content-Type: application/json" \
     -d '{"title": "スクリプトによる自動生成された記事", "content": "これはスクリプトによって自動生成された記事です。", "subcategory_id": 1}'
echo

sleep 1

echo "特定の記事の取得"
echo "GET /api/articles/{slug} slug=5de294e1-ef3a-11ee-a436-00155d3cfd23"
curl -X GET "http://localhost:8080/api/articles/5de294e1-ef3a-11ee-a436-00155d3cfd23" 

sleep 1

# echo "記事一覧の取得(追加されていることの確認)"
# echo "GET /api/articles"
# curl -X GET "http://localhost:8080/api/articles"
# echo "\n"

