import {
  Card,
  CardHeader,
  CardBody,
  Text,
  Heading,
} from "../../common/components";
import { Article } from "../../types";
import ReactMarkdown from 'react-markdown';
import remarkGfm from 'remark-gfm';
import remarkMath from 'remark-math';
import rehypeKatex from 'rehype-katex';
import './markdown.css';
import 'katex/dist/katex.min.css';

export default function ArticleContent({ article }: { article: Article }) {

  return (
    <Card as="article">
      <CardHeader>
        <Heading as="h1">{article.title}</Heading>
      </CardHeader>
      <CardBody>
        <div>
          <ReactMarkdown className={"markdown"} remarkPlugins={[remarkMath, remarkGfm]} rehypePlugins={[rehypeKatex]}>
            {article.content}
          </ReactMarkdown>
        </div>
      </CardBody>
    </Card>
  );
}