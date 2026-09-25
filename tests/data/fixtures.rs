// The fixture corpus: `name url rev`, cloned to `tests/fixtures/<name>` with
// `_` spelled `-` in the directory name. Included by
// examples/clone_fixtures.rs and tests/fixture_baselines.rs, which each define
// their own `fixtures!` macro. Every fixture is registered as a test: training
// fixtures get a full divergence report, validation fixtures are held out
// (see `Skill(iterate-divergence)`).

fixtures! {
    training {
        // Rust
        anyhow               "https://github.com/dtolnay/anyhow.git"                  "769cba0b",
        thiserror            "https://github.com/dtolnay/thiserror.git"               "9ac165c4",
        log                  "https://github.com/rust-lang/log.git"                   "43f2c283",
        mdbook               "https://github.com/rust-lang/mdBook.git"                "b8c90970",
        toasty               "https://github.com/tokio-rs/toasty.git"                 "0fb6be95",
        sps                  "https://github.com/alexykn/sps.git"                     "5a10e7f4",
        otree                "https://github.com/fioncat/otree.git"                   "a02bdf44",
        hyperfine            "https://github.com/sharkdp/hyperfine.git"               "f12f3d9f",
        // Go
        go_multierror        "https://github.com/hashicorp/go-multierror.git"         "edef97ed",
        xxhash               "https://github.com/cespare/xxhash.git"                  "ab37246c",
        mcphost              "https://github.com/mark3labs/mcphost.git"               "191dcea1",
        tock                 "https://github.com/kriuchkov/tock.git"                  "b29815f2",
        cobra                "https://github.com/spf13/cobra.git"                     "ad460ea8",
        gin                  "https://github.com/gin-gonic/gin.git"                   "5f4f9643",
        lo                   "https://github.com/samber/lo.git"                       "ea94cc5c",
        migrate              "https://github.com/golang-migrate/migrate.git"          "2bd822b3",
        bubbletea            "https://github.com/charmbracelet/bubbletea.git"         "c60f0c53",
        mkcert               "https://github.com/FiloSottile/mkcert.git"              "1c1dc4ed",
        act                  "https://github.com/nektos/act.git"                      "123167dc",
        beszel               "https://github.com/henrygd/beszel.git"                  "c1c1cd1b",
        // TypeScript
        cmdk                 "https://github.com/pacocoursey/cmdk.git"                "dd2250ed",
        vaul                 "https://github.com/emilkowalski/vaul.git"               "3e97aac6",
        ts_pattern           "https://github.com/gvergnaud/ts-pattern.git"            "2ece6ba5",
        ky                   "https://github.com/sindresorhus/ky.git"                 "eb5c3eba",
        superstruct          "https://github.com/ianstormtaylor/superstruct.git"      "e414c8af",
        mitt                 "https://github.com/developit/mitt.git"                  "6b416705",
        enclosed             "https://github.com/CorentinTh/enclosed.git"             "461c3d41",
        d2ts                 "https://github.com/electric-sql/d2ts.git"               "418591d5",
        p_queue              "https://github.com/sindresorhus/p-queue.git"            "fc4b7369",
        linkwarden           "https://github.com/linkwarden/linkwarden.git"           "22723575",
        vite                 "https://github.com/vitejs/vite.git"                     "b3132dac",
        monaco_editor        "https://github.com/microsoft/monaco-editor.git"         "6c8488c6",
        // JavaScript
        commander            "https://github.com/tj/commander.js.git"                 "82473649",
        semver               "https://github.com/npm/node-semver.git"                 "5993c2e4",
        express              "https://github.com/expressjs/express.git"               "f873ac23",
        axios                "https://github.com/axios/axios.git"                     "26a87296",
        chalk                "https://github.com/chalk/chalk.git"                     "aa06bb5a",
        debug                "https://github.com/debug-js/debug.git"                  "f405ade8",
        json_server          "https://github.com/typicode/json-server.git"            "89a34a44",
        svgo                 "https://github.com/svg/svgo.git"                        "581fe687",
        dockly               "https://github.com/lirantal/dockly.git"                 "a817de79",
        audiobookshelf       "https://github.com/advplyr/audiobookshelf.git"          "eee377e0",
        // Python
        pluggy               "https://github.com/pytest-dev/pluggy.git"               "4cc08c15",
        typeguard            "https://github.com/agronholm/typeguard.git"             "b05b7dab",
        tomli                "https://github.com/hukkin/tomli.git"                    "920e20b1",
        peepdb               "https://github.com/evangelosmeklis/peepdb.git"          "929064dd",
        swarm                "https://github.com/openai/swarm.git"                    "0c82d7d8",
        htmy                 "https://github.com/volfpeter/htmy.git"                  "4694fb86",
        microbootstrap       "https://github.com/community-of-python/microbootstrap.git" "609c420b",
        py3xui               "https://github.com/iwatkot/py3xui.git"                  "6004c163",
        flask                "https://github.com/pallets/flask.git"                   "9fcd34c9",
        requests             "https://github.com/psf/requests.git"                    "6e83187b",
        rich                 "https://github.com/Textualize/rich.git"                 "46cebbb0",
        click                "https://github.com/pallets/click.git"                   "c943271a",
        beets                "https://github.com/beetbox/beets.git"                   "5df37abc",
        posting              "https://github.com/darrenburns/posting.git"             "56703a11",
        linkding             "https://github.com/sissbruecker/linkding.git"           "573b6f54",
        // Python (ML)
        xlstm                "https://github.com/NX-AI/xlstm.git"                     "032a6fb8",
        nano_vllm            "https://github.com/GeeeekExplorer/nano-vllm.git"        "2f214426",
        chronos_forecasting  "https://github.com/amazon-science/chronos-forecasting.git" "f951d9ae",
        // C
        sds                  "https://github.com/antirez/sds.git"                     "5347739b",
        neco                 "https://github.com/tidwall/neco.git"                    "9e8e19e4",
        bareiron             "https://github.com/p2r3/bareiron.git"                   "ddb071c3",
        krep                 "https://github.com/davidesantangelo/krep.git"           "ae96fbd2",
        sqlite_vec           "https://github.com/asg017/sqlite-vec.git"               "563a3e60",
        soluna               "https://github.com/cloudwu/soluna.git"                  "be822052",
        htop                 "https://github.com/htop-dev/htop.git"                   "b7f9df97",
        jq                   "https://github.com/jqlang/jq.git"                       "f58787c4",
        tinyusb              "https://github.com/hathach/tinyusb.git"                 "7f146c9f",
        chibicc              "https://github.com/rui314/chibicc.git"                  "90d1f7f1",
        // Lua
        middleclass          "https://github.com/kikito/middleclass.git"              "359f0e27",
    }
    validation {
        // Python
        aiogram              "https://github.com/aiogram/aiogram.git"                 "f6b2cd53",
        mkdocs               "https://github.com/mkdocs/mkdocs.git"                   "28625367",
        healthchecks         "https://github.com/healthchecks/healthchecks.git"       "ac7b523a",
        statsmodels          "https://github.com/statsmodels/statsmodels.git"         "3c102982",
        xonsh                "https://github.com/xonsh/xonsh.git"                     "70212600",
        httpie               "https://github.com/httpie/cli.git"                      "5b604c37",
        crawlee_python       "https://github.com/apify/crawlee-python.git"            "b3b8c59b",
        // TypeScript
        pretty_ts_errors     "https://github.com/yoavbls/pretty-ts-errors.git"        "980c274b",
        drizzle_orm          "https://github.com/drizzle-team/drizzle-orm.git"        "48e54060",
        preact_signals       "https://github.com/preactjs/signals.git"                "cca91a3c",
        clack                "https://github.com/bombshell-dev/clack.git"             "adb6af9f",
        excalidraw           "https://github.com/excalidraw/excalidraw.git"           "f6d85bc8",
        // JavaScript
        marked               "https://github.com/markedjs/marked.git"                 "a7affc3b",
        handlebars           "https://github.com/handlebars-lang/handlebars.js.git"   "3105ca73",
        octotree             "https://github.com/ovity/octotree.git"                  "470747c7",
        rough_viz            "https://github.com/jwilber/roughViz.git"                "17c7ea86",
        // Go
        helm                 "https://github.com/helm/helm.git"                       "b2786f15",
        nats_server          "https://github.com/nats-io/nats-server.git"             "8f41bed8",
        rqlite               "https://github.com/rqlite/rqlite.git"                   "c74c936d",
        // C
        mongoose             "https://github.com/cesanta/mongoose.git"                "1d373ce5",
        sameboy              "https://github.com/LIJI32/SameBoy.git"                  "208ba4af",
        // Rust
        xh                   "https://github.com/ducaale/xh.git"                      "b928cf08",
    }
}
