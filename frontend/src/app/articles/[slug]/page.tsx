import { notFound } from "next/navigation";
import { Article, Comment } from "../../types";
import { Suspense } from "react";
import ArticleContent from "./ArticleContent";
import Comments from "./Comments";
import { Heading } from "../../common/components";
import LoadingComments from "./LoadingComments";
// import LatexToHtmlConverter from "../../LatexToHtmlConverter";

const getArticle = async (slug: string) => {
  const res = await fetch(`http://localhost:8080/api/articles/${slug}`, {
    next: { revalidate: 60 },
  });

  if (res.status === 404) {
    // notFound 関数を呼び出すと not-fount.tsx を表示する
    notFound();
  }

  if (!res.ok) {
    throw new Error("Failed to fetch article");
  }

  const data = await res.json();
  return data as Article;
};

const getComments = async (slug: string) => {
  const res = await fetch(
    `http://localhost:8080/api/articles/${slug}/comments`,
    {
      cache: "no-store",
    }
  );

  if (!res.ok) {
    throw new Error("Failed to fetch comments");
  }

  const data = await res.json();
  return data as Comment[];
};

export default async function ArticleDetail({
  params,
}: {
  params: { slug: string };
}) {
  const articlePromise = getArticle(params.slug);
  const commentPromise = getComments(params.slug);

  const article = await articlePromise;

  return (
    <div>
      <ArticleContent article={article} />

      <Heading as="h2" mt={8} mb={4}>
        Comments
      </Heading>
      <Suspense fallback={<LoadingComments />}>
        <Comments commentPromise={commentPromise} />
      </Suspense>
    </div>
  );
}
