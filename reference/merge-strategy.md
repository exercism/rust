# merging strategy

The Rust track maintainers [have decided](https://github.com/exercism/v3/discussions/1725#discussion-7438) to write down our merging strategy to communicate expectations and encourage quality contributions.

## general principles

- we strive to accept _all_ contributions
  - We'll bring contributions to a high standard of quality through collaborative, iterative review.
  - We adopt a pessimistic merging approach. Think of maintainer reviews like the Rust compiler's feedback. The amount of feedback can feel overwhelming and frustrating. But we'll work with you to get it over the finish line! So keep up the good work.
- Every pull request must have at least one "Approval", and no "Request Changes", from a track maintainer before it can be merged.
- No maintainer can approve their own pull request.

## pull request workflow

### definitions

| Term               |                                                                                               Meaning |
| ------------------ | ----------------------------------------------------------------------------------------------------: |
| Contributor        |                                                                     Anyone submitting a pull request. |
| Maintainer         |         Anyone who belongs to the [@exercism/rust team](https://github.com/orgs/exercism/teams/rust). |
| development branch | The [v3](https://github.com/exercism/v3) repository's default development branch. Currently `master`. |

### the workflow

1. Contributors start by opening a [draft](https://github.blog/2019-02-14-introducing-draft-pull-requests/) or a complete pull request with their changes
   1. Using the draft feature signals the work is ready for high-level review, but that details may currently be unimplemented, incomplete, or wrong.
   1. After an unspecified number of iterations of steps 2-3, the contributor marks the pull request as ready for review.
1. One other maintainer shall review the contribution and provide feedback.
1. The contributor shall address _all_ maintainer feedback as part of the pull request.
   1. Maintainers shall consider using GitHub's suggestion feature for nitpick or cut-and-dry changes.
   1. Pushback is encouraged when a contributor believes that they are right and a maintainer is wrong about a feedback item. "Addressing" a piece of feedback is not synonymous with slavishly implementing it.
1. The maintainer shall review the feedback and either:
   1. approve
   1. request additional changes. At this point, the process returns to step 3.
1. Upon approval, a maintainer shall merge their pull request.
   1. Maintainers shall merge their own pull requests. Since contributors are not necessarily maintainers, a maintainer must preform this step.
