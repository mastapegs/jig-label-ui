import { LitElement, html, css } from "lit";
import { customElement, property } from "lit/decorators";
import { JigData } from "./types";

@customElement("jib-label-ui")
export class JigLabelUI extends LitElement {
  static styles = css`
    .container {
      font-family: sans-serif;
    }
    .headers {
      display: flex;
      justify-content: space-between;
    }
    .jig {
      display: flex;
      justify-content: space-between;
    }
    .flex {
      flex: 1 1 0;
    }
  `;

  @property({ attribute: false })
  headers: string[] = [
    "Jig Name",
    "Author",
    "Author's Badge",
    "Date",
    "Instruction Language",
    "Curators",
  ];

  @property({ attribute: false })
  jigs: JigData[] = [];

  render() {
    return html`
      <div class="container">
        <div class="headers">
          ${this.headers.map(
            (header) => html`<div class="flex">${header}</div>`
          )}
        </div>
        ${this.jigs.map((jig) => {
          return html`<div class="jig">
            <div class="flex">${jig.jig_name}</div>
            <div class="flex">${jig.author}</div>
            <div class="flex">${jig.author_badge}</div>
            <div class="flex">${jig.date}</div>
            <div class="flex">${jig.language}</div>
            <div class="flex">${jig.curators.join(", ")}</div>
          </div>`;
        })}
      </div>
    `;
  }
}
