<div align="center">
  <picture>
    <source media="(prefers-color-scheme: dark)" srcset="https://raw.githubusercontent.com/checkjs/resources/main/svg/slogan-dark-transparent.svg">
    <source media="(prefers-color-scheme: light)" srcset="https://raw.githubusercontent.com/checkjs/resources/main/svg/slogan-light-transparent.svg">
    <img alt="Muestra el banner de Check, con su respectivo logo y la frase 'Check - Toolchain of the web'." src="https://raw.githubusercontent.com/checkjs/resources/main/svg/slogan-light-transparent.svg" width="700">
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

  [हिन्दी](https://github.com/checkjs/check/blob/main/packages/%40checkjs/check/README.hi.md) | [English](https://github.com/checkjs/check/blob/main/packages/%40checkjs/check/README.md) | Español | [Français](https://github.com/checkjs/check/blob/main/packages/%40checkjs/check/README.fr.md) | [繁體中文](https://github.com/checkjs/check/blob/main/packages/%40checkjs/check/README.zh-TW.md) | [简体中文](https://github.com/checkjs/check/blob/main/packages/%40checkjs/check/README.zh-CN.md) | [日本語](https://github.com/checkjs/check/blob/main/packages/%40checkjs/check/README.ja.md) | [Português do Brasil](https://github.com/checkjs/check/blob/main/packages/%40checkjs/check/README.pt-BR.md) | [한국어](https://github.com/checkjs/check/blob/main/packages/%40checkjs/check/README.kr.md) | [Русский](https://github.com/checkjs/check/blob/main/packages/%40checkjs/check/README.ru.md) | [Українська](https://github.com/checkjs/check/blob/main/packages/%40checkjs/check/README.uk.md)
</div>

<br>

**Check** es una toolchain de alto rendimiento para proyectos web. Su objetivo es proveer herramientas que mantengan la salud de dichos proyectos.

**Check es un [formatter rápido](https://github.com/checkjs/check/tree/main/benchmark#formatting)** para _JavaScript_, _TypeScript_, _JSX_, _JSON_, _CSS_ y _GraphQL_ que alcanza un **[97% de compatibilidad con _Prettier_](https://console.algora.io/challenges/prettier)**.

**Check es un [linter de alto rendimiento](https://github.com/checkjs/check/tree/main/benchmark#linting)** para _JavaScript_, _TypeScript_, _JSX_, _JSON_, _CSS_, y _GraphQL_ que incluye **[más de 340 reglas](https://checkjs.dev/linter/javascript/rules/)** de ESLint, typescript-eslint, y [otras fuentes](https://github.com/checkjs/check/discussions/3).
Produce **diagnósticos detallados y contextualizados** que ayudan a mejorar tu código y convertirte en un mejor programador!

**Check** fue diseñado desde cero para usarse [interactivamente dentro de un editor](https://checkjs.dev/guides/editors/first-party-extensions/).
Puede formatear y analizar código mal formado mientras lo estás escribiendo.

### Instalación

```shell
npm install --save-dev --save-exact @checkjs/check
```

### Uso

```shell
# formatea archivos
npx @checkjs/check format --write

# analiza archivos y aplica correcciones seguras
npx @checkjs/check lint --write

# ejecuta format, lint, etc. y aplica correcciones seguras
npx @checkjs/check check --write

# valida todos los archivos con format, lint, etc. en entornos CI
npx @checkjs/check ci
```

Si querés probar Check sin instalarlo, usá el [playground online](https://checkjs.dev/playground/), compilado a WebAssembly.

## Documentación

Visitá nuestra [página principal][checkjs] para aprender más sobre Check,
o andá directamente a la [guía de inicio][getting-started] para empezar a usarlo.

## Más sobre Check

**Check** tiene valores predeterminados sanos y no requiere configuración.

**Check** busca soportar [todos los lenguajes principales][language-support] del desarrollo web moderno.

**Check** [no necesita Node.js](https://checkjs.dev/guides/manual-installation/) para funcionar.
**Check** tiene soporte de primera clase para LSP, con un parser sofisticado que representa el texto fuente con total fidelidad y excelente recuperación de errores.

**Check** apunta a ofrecer una *Experiencia de Desarrollador* de alta calidad, con diagnósticos descriptivos y gran rendimiento.

**Check** unifica funcionalidades que antes estaban en herramientas separadas. Construir sobre una base compartida nos permite dar una experiencia cohesiva para procesar código, mostrar errores, paralelizar trabajo, usar caché y manejar configuración.

Leé más sobre nuestra [filosofía del proyecto][check-philosophy].

**Check** está bajo licencia [MIT](https://github.com/checkjs/check/tree/main/LICENSE-MIT) o [Apache 2.0 licensed](https://github.com/checkjs/check/tree/main/LICENSE-APACHE)y moderado bajo el [Código de Conducta](https://github.com/checkjs/check/tree/main/CODE_OF_CONDUCT.md).

## Financiamiento

Podés financiar el proyecto de distintas maneras

### Patrocinios y financiamiento

Podés patrocinar o financiar el proyecto a través de [Open collective](https://opencollective.com/check) o [GitHub sponsors](https://github.com/sponsors/checkjs)

Check ofrece un programa de patrocinio simple que permite a las empresas obtener visibilidad y reconocimiento entre varios desarrolladores.

Check ofrece [soporte empresarial](https://checkjs.dev/enterprise), donde colaboradores principales pueden ser contratados para trabajar en proyectos enfocados a compañías.

## Sponsors

### Sponsors Oro

<table>
  <tbody>
    <tr>
      <td align="center" valign="middle">
        <a href="https://depot.dev/?utm_source=check&utm_medium=readme" target="_blank">
          <picture>
            <source media="(prefers-color-scheme: light)" srcset="https://depot.dev/assets/brand/1693758816/depot-logo-horizontal-on-light@3x.png" />
            <source media="(prefers-color-scheme: dark)" srcset="https://depot.dev/assets/brand/1693758816/depot-logo-horizontal-on-dark@3x.png" />
            <img src="https://depot.dev/assets/brand/1693758816/depot-logo-horizontal-on-light@3x.png" width="400" alt="Depot logo" />
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

### Sponsors Plata

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

### Sponsors Bronce

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
