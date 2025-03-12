use clap::{Parser, ValueEnum};
use time::UtcOffset;

#[derive(Parser)]
#[command(version, about, author, long_about = None, help_template = "\
{before-help}{name} v{version}

{about}
{tab}{author}

{usage-heading} {usage}

{all-args}{after-help}
")]
struct Args {
    timestamp: i128,
    #[arg(long, short, env = "TSTD_RESOLUTION", default_value = "millis")]
    /// Resolution of the timestamp
    resolution: Resolution,

    #[arg(long, short = 'i', default_value_t = 0)]
    /// Offset of the <TIMESTAMP> from UTC in hours
    input_offset: i8,

    #[arg(long, short = 'o', default_value = "local")]
    /// Which timezone to display the parsed datetime in
    output_offset: OutputOffset,
}

#[derive(Clone, Copy, ValueEnum)]
enum Resolution {
    Secs,
    Millis,
    Nanos,
}

#[derive(Clone, Copy, ValueEnum)]
enum OutputOffset {
    Local,
    Utc,
    Original,
}

fn main() -> Result<(), &'static str> {
    let args = Args::parse();

    let input_offset =
        UtcOffset::from_hms(args.input_offset, 0, 0).map_err(|_| "Invalid offset")?;

    let datetime = match args.resolution {
        Resolution::Secs => time::OffsetDateTime::from_unix_timestamp(
            args.timestamp
                .try_into()
                .map_err(|_| "Timestamp out of range for selected resolution")?,
        )
        .map_err(|_| "Could not parse time")?,

        Resolution::Millis => {
            time::OffsetDateTime::from_unix_timestamp_nanos(args.timestamp * 1000000)
                .map_err(|_| "Could not parse time")?
        }

        Resolution::Nanos => time::OffsetDateTime::from_unix_timestamp_nanos(args.timestamp)
            .map_err(|_| "Could not parse time")?,
    }
    .replace_offset(input_offset);

    let datetime = match args.output_offset {
        OutputOffset::Local => datetime.to_offset(
            UtcOffset::current_local_offset().map_err(|_| "Can't get local UTC offset")?,
        ),

        OutputOffset::Utc => datetime.to_offset(UtcOffset::UTC),

        OutputOffset::Original => datetime,
    };

    println!(
        "{}",
        datetime
            .format(&time::format_description::well_known::Rfc2822)
            .map_err(|_| "Couldn't format datetime")?
    );

    Ok(())
}
