import {
    Card,
    CardHeader,
    CardBody,
    Text,
    Heading,
  } from "../../common/components";
  import { Article } from "../../types";
  // import { BlockMath } from "react-katex";
  import ReactMarkdown from 'react-markdown';
  // import remarkGfm from 'remark-gfm';
  import remarkMath from 'remark-math';
  import rehypeKatex from 'rehype-katex';

  export default function ArticleContent({ article }: { article: Article }) {
    const markdownString = `
# Markdown

Lorem ipsum dolor sit amet consectetur, adipisicing elit. Magni, nemo!
`;
    return (
      <ReactMarkdown>
          {markdownString}
        </ReactMarkdown>
      // <Card as="article">
      //   <CardHeader>
      //     <Heading as="h1">{article.title}</Heading>
      //   </CardHeader>
      //   <CardBody>
      //   <ReactMarkdown remarkPlugins={[remarkMath]} rehypePlugins={[rehypeKatex]}>
      //     {article.content}
      //   </ReactMarkdown>
      //     <Text as="p" fontSize="md">
      //       {article.content}
      //     </Text>
      //   </CardBody>
      // </Card>
    );
  }