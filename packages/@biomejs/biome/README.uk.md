<div align="center">
  <picture>
    <source media="(prefers-color-scheme: dark)" srcset="https://raw.githubusercontent.com/checkjs/resources/main/svg/slogan-dark-transparent.svg">
    <source media="(prefers-color-scheme: light)" srcset="https://raw.githubusercontent.com/checkjs/resources/main/svg/slogan-light-transparent.svg">
    <img alt="Shows the banner of Check, with its logo and the phrase 'Check - Toolchain of the web'." src="https://raw.githubusercontent.com/checkjs/resources/main/svg/slogan-light-transparent.svg" width="700">
  </picture>

  <br>
  <br>

  [![CI on main][ci-badge]][ci-url]
  [![Discord chat][discord-badge]][discord-url]
  [![npm version][npm-badge]][npm-url]
  [![VSCode version][vscode-badge]][vscode-url]
  [![Open VSX version][open-vsx-badge]][open-vsx-url]

  [ci-badge]: https://github.com/checkjs/check/actions/workflows/main.yml/badge.svg
  [ci-url]: https://github.com/checkjs/check/actions/workflows/main.yml
  [discord-badge]: https://badgen.net/discord/online-members/BypW39g6Yc?icon=discord&label=discord&color=60a5fa
  [discord-url]: https://checkjs.dev/chat
  [npm-badge]: https://badgen.net/npm/v/@checkjs/check?icon=npm&color=60a5fa&label=%40checkjs%2Fcheck
  [npm-url]: https://www.npmjs.com/package/@checkjs/check/v/latest
  [vscode-badge]: https://img.shields.io/visual-studio-marketplace/v/checkjs.check?label=Visual%20Studio%20Marketplace&labelColor=374151&color=60a5fa
  [vscode-url]: https://marketplace.visualstudio.com/items?itemName=checkjs.check
  [open-vsx-badge]: https://img.shields.io/visual-studio-marketplace/v/checkjs.check?label=Open%20VSX%20Registry&logo=data:image/svg+xml;base64,PD94bWwgdmVyc2lvbj0iMS4wIiBlbmNvZGluZz0idXRmLTgiPz4KPHN2ZyB2aWV3Qm94PSI0LjYgNSA5Ni4yIDEyMi43IiB4bWxucz0iaHR0cDovL3d3dy53My5vcmcvMjAwMC9zdmciPgogIDxwYXRoIGQ9Ik0zMCA0NC4yTDUyLjYgNUg3LjN6TTQuNiA4OC41aDQ1LjNMMjcuMiA0OS40em01MSAwbDIyLjYgMzkuMiAyMi42LTM5LjJ6IiBmaWxsPSIjYzE2MGVmIi8+CiAgPHBhdGggZD0iTTUyLjYgNUwzMCA0NC4yaDQ1LjJ6TTI3LjIgNDkuNGwyMi43IDM5LjEgMjIuNi0zOS4xem01MSAwTDU1LjYgODguNWg0NS4yeiIgZmlsbD0iI2E2MGVlNSIvPgo8L3N2Zz4=&labelColor=374151&color=60a5fa
  [open-vsx-url]: https://open-vsx.org/extension/checkjs/check

  <!-- Insert new entries lexicographically by language code.
     For example given below is the same order as these files appear on page:
     https://github.com/checkjs/check/tree/main/packages/@checkjs/check -->

  [हिन्दी](https://github.com/checkjs/check/blob/main/packages/%40checkjs/check/README.hi.md) | [English](https://github.com/checkjs/check/blob/main/packages/%40checkjs/check/README.md) | [Español](https://github.com/checkjs/check/blob/main/packages/%40checkjs/check/README.es.md) | [Français](https://github.com/checkjs/check/blob/main/packages/%40checkjs/check/README.fr.md) | [繁體中文](https://github.com/checkjs/check/blob/main/packages/%40checkjs/check/README.zh-TW.md) | [简体中文](https://github.com/checkjs/check/blob/main/packages/%40checkjs/check/README.zh-CN.md) | [日本語](https://github.com/checkjs/check/blob/main/packages/%40checkjs/check/README.ja.md) | [Português do Brasil](https://github.com/checkjs/check/blob/main/packages/%40checkjs/check/README.pt-BR.md) | [한국어](https://github.com/checkjs/check/blob/main/packages/%40checkjs/check/README.kr.md)| [Русский](https://github.com/checkjs/check/blob/main/packages/%40checkjs/check/README.ru.md) | Українська
</div>

<br>

**Check** - це високопродуктивний інструментарій для веб-проєктів, який має на меті надавати інструменти розробникам для підтримки здоров'я проєктів.

**Check - це [швидкий форматувальник](./benchmark#formatting)** для _JavaScript_, _TypeScript_, _JSX_, _JSON_, _CSS_ та _GraphQL_, який досягає **[97% сумісності з _Prettier_](https://console.algora.io/challenges/prettier)**.

**Check - це [високопродуктивний лінтер](https://github.com/checkjs/check/tree/main/benchmark#linting)** для _JavaScript_, _TypeScript_, _JSX_, _CSS_ та _GraphQL_, який містить **[понад 340 правил](https://checkjs.dev/uk/linter/javascript/rules/)** з ESLint, typescript-eslint та [інших джерел](https://github.com/checkjs/check/discussions/3).
Він **виводить детальні та контекстуалізовані діагностичні дані**, які допомагають вам покращити ваш код та стати кращим програмістом!

**Check** з самого початку розроблений для [інтерактивного використання в редакторі](https://checkjs.dev/guides/editors/first-party-extensions/). Він може форматувати та лінтити некоректний код під час його написання.

### Встановлення

```shell
npm install --save-dev --save-exact @checkjs/check
```

### Використання

```shell
# форматування файлів
npx @checkjs/check format --write ./src

# лінтинг файлів та застосування безпечних виправлень
npx @checkjs/check lint --write ./src

# запуск форматування, лінтингу тощо та застосування безпечних виправлень
npx @checkjs/check check --write ./src

# перевірка всіх файлів на відповідність форматуванню, лінтингу тощо в середовищах CI
npx @checkjs/check ci ./src
```

Якщо ви хочете спробувати Check без встановлення, скористайтеся [онлайн-пісочницею](https://checkjs.dev/playground/), скомпільованою у WebAssembly.

## Документація

Перегляньте нашу [домашню сторінку][checkjs], щоб дізнатися більше про Check,
або перейдіть безпосередньо до [посібника з початку роботи][getting-started], щоб почати використовувати Check.

## Більше про Check

**Check** має розумні налаштування за замовчуванням і не потребує конфігурації.

**Check** прагне підтримувати [всі основні мови][language-support] сучасної веб-розробки.

**Check** [не потребує Node.js](https://checkjs.dev/guides/manual-installation/) для роботи.

**Check** має першокласну підтримку LSP, з витонченим парсером, який представляє вихідний текст з повною точністю та найкращим відновленням помилок.

**Check** об'єднує функціональність, яка раніше була окремими інструментами. Побудова на спільній основі дозволяє нам забезпечити узгоджений досвід обробки коду, відображення помилок, паралельної роботи, кешування та конфігурації.

Дізнайтеся більше про нашу [філософію проєкту][check-philosophy].

**Check** має [ліцензію MIT](https://github.com/checkjs/check/tree/main/LICENSE-MIT) або [ліцензію Apache 2.0](https://github.com/checkjs/check/tree/main/LICENSE-APACHE) і регулюється [Кодексом поведінки учасників](https://github.com/checkjs/check/tree/main/CODE_OF_CONDUCT.md).

## Фінансування

Ви можете фінансувати проєкт різними способами

### Спонсорство та фінансування проєкту

Ви можете спонсорувати або фінансувати проєкт через [Open collective](https://opencollective.com/check) або [GitHub sponsors](https://github.com/sponsors/checkjs)

Check пропонує просту програму спонсорства, яка дозволяє компаніям отримувати видимість та визнання серед різних розробників.

### Фінансування завдань

Ми використовуємо [Polar.sh](https://polar.sh/checkjs) для голосування та просування конкретних функцій, які ви хотіли б бачити та реалізувати. Перевірте наш список завдань і допоможіть нам:

## Спонсори

### Золоті спонсори

<table>
  <tbody>
    <tr>
      <td align="center" valign="middle">
        <a href="https://depot.dev/?utm_source=check&utm_medium=readme" target="_blank">
          <picture>
            <source media="(prefers-color-scheme: light)" srcset="https://depot.dev/assets/brand/1693758816/depot-logo-horizontal-on-light@3x.png" />
            <source media="(prefers-color-scheme: dark)" srcset="https://depot.dev/assets/brand/1693758816/depot-logo-horizontal-on-dark@3x.png" />
            <img src="https://depot.dev/assets/brand/1693758816/depot-logo-horizontal-on-light@3x.png" width="400" alt="Depot" />
          </picture>
        </a>
      </td>
    </tr>
    <tr>
      <td align="center" valign="middle">
        <a href="https://vercel.com/?utm_source=check&utm_medium=readme" target="_blank">
          <picture>
            <source media="(prefers-color-scheme: light)" srcset="https://raw.githubusercontent.com/checkjs/resources/refs/heads/main/sponsors/vercel-dark.png" />
            <source media="(prefers-color-scheme: dark)" srcset="https://raw.githubusercontent.com/checkjs/resources/refs/heads/main/sponsors/vercel-light.png" />
            <img src="https://raw.githubusercontent.com/checkjs/resources/refs/heads/main/sponsors/vercel-dark.png" width="400" alt="Vercel" />
          </picture>
        </a>
      </td>
    </tr>
  </tbody>
</table>

### Срібні спонсори

<table>
  <tbody>
    <tr>
      <td align="center" valign="middle">
        <a href="https://l2beat.com/?utm_source=check&utm_medium=readme" target="_blank"><img src="https://images.opencollective.com/l2beat/c2b2a27/logo/256.png" height="100" alt="L2BEAT logo"></a>
      </td>
      <td align="center" valign="middle">
        <a href="https://www.phoenixlabs.dev/?utm_source=check&utm_medium=readme" target="_blank"><img src="https://images.opencollective.com/phoenix-labs/2824ed4/logo/100.png?height=100" height="100" alt="Phoenix Labs logo"></a>
      </td>
      <td align="center" valign="middle">
        <a href="https://lokalise.com/?utm_source=check&utm_medium=readme" target="_blank"><img src="https://avatars.githubusercontent.com/u/14294501?s=200&v=4" height="100" alt="Lokalise logo"></a>
      </td>
    </tr>
  </tbody>
</table>
### Бронзові спонсори

<table>
  <tbody>
    <tr>
      <td align="center" valign="middle">
        <a href="https://nanabit.dev/?utm_source=check&utm_medium=readme" target="_blank"><img src="https://images.opencollective.com/nanabit/d15fd98/logo/256.png?height=80" width="80" alt="Nanabit logo"></a>
      </td>
      <td align="center" valign="middle">
        <a href="https://vital.io/?utm_source=check&utm_medium=readme" target="_blank"><img src="https://avatars.githubusercontent.com/u/25357309?s=200" width="80" alt="Vital logo"></a>
      </td>
      <td align="center" valign="middle">
        <a href="https://coderabbit.ai/?utm_source=check&utm_medium=readme" target="_blank"><img src="https://avatars.githubusercontent.com/u/132028505?s=200&v=4" width="80" alt="CodeRabbit logo"></a>
      </td>
      <td align="center" valign="middle">
        <a href="https://forge42.dev/?utm_source=check&utm_medium=readme" target="_blank"><img src="https://avatars.githubusercontent.com/u/161314831?s=200&v=4" width="80" alt="Forge42 logo"></a>
      </td>
      <td align="center" valign="middle">
        <a href="http://rstudio.org/?utm_source=check&utm_medium=readme" target="_blank"><img src="https://avatars.githubusercontent.com/u/513560?s=200&v=4" width="80" alt="RStudio logo"></a>
      </td>
      <td align="center" valign="middle">
        <a href="https://pennylane.com/?utm_source=check&utm_medium=readme" target="_blank"><img src="https://avatars.githubusercontent.com/u/57875210?s=200&v=4" width="80" alt="Pennylane logo"></a>
      </td>
      <td align="center" valign="middle">
        <a href="https://jetbrains.com/?utm_source=check&utm_medium=readme" target="_blank"><img src="https://resources.jetbrains.com/storage/products/company/brand/logos/jetbrains.png" width="100" alt="JetBrains logo"></a>
      </td>
      <td align="center" valign="middle">
        <a href="https://www.egstock.co.jp/?utm_source=check&utm_medium=readme" target="_blank"><img src="https://images.opencollective.com/egstock/b18c836/logo/256.png?height=256" width="80" alt="EGSTOCK, Inc. logo"></a>
      </td>
    </tr>
  </tbody>
</table>


[checkjs]: https://checkjs.dev/
[check-philosophy]: https://checkjs.dev/internals/philosophy/
[language-support]: https://checkjs.dev/internals/language-support/
[getting-started]: https://checkjs.dev/guides/getting-started/
