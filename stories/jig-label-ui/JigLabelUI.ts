import { LitElement, html } from "lit";
import { customElement, property } from "lit/decorators";

interface JigData {
  jig_name: string;
  author: string;
  author_badge: string;
  date: string;
  language: string;
  curators: string[];
}

@customElement("jib-label-ui")
export class JigLabelUI extends LitElement {
  @property({ attribute: false })
  headers: string[] = ["1", "2"];

  @property({ attribute: false })
  jigs: JigData[] = [];

  render() {
    return html`<ul>
      ${this.headers.map((header) => html`<li>${header}</li>`)}
    </ul>`;
  }
}
