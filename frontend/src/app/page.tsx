import type { Article } from "./types";
import ArticleList from "./ArticleList";
import { Heading } from "./common/components";

async function getArticles() {
  const res = await fetch("http://localhost:8080/api/articles", {
    cache: "no-store",
  });

  // throw new Error("Failed to fetch articles");

  // エラーハンドリングを行うことが推奨されている
  if (!res.ok) {
    throw new Error("Failed to fetch articles");
  }

  // const hey = await fetch("http://127.0.0.1:8080/hey");
  // if (!hey.ok) {
  //   throw new Error("Failed to fetch hey");
  // }e
  // console.log(await hey.text()); // レスポンスボディをテキストとして読み込む

  const data = await res.json();
  return data.articles as Article[];
}

export default async function Home() {
  const articles = await getArticles();

  return (
    <div>
      <Heading as="h1" mb={4}>
        新着記事
      </Heading>
      <ArticleList articles={articles} />
    </div>
  );
}
