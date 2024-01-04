import { getProfileProgress } from "./profile"
import { constructRule } from "./util"

export async function constructMultiRule(
    profiles: MultiProfileProgress["profiles"],
): Promise<CompositeRule> {
    const rawResults = await Promise.all(
        profiles.map(async (p) => ({
            ...p,
            progress: await getProfileProgress(p.id),
        })),
    )
    return {
        type: "Composite",
        rules: rawResults
            .filter((p) => !!p.progress)
            .map(({ condition, progress }) => ({
                condition,
                rule: constructRule(progress!.rules),
            })),
    }
}
