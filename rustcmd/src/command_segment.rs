use std::{borrow::Cow, env::VarError};

use crate::{env_unset_handler::EnvUnsetHandler, ExecuteError};

pub enum CommandSegment {
    SplitWhitespace(String),
    Literally(String),
    Env {
        name: String,
        if_unset: EnvUnsetHandler,
    },
}

impl CommandSegment {
    // TODO: you know, maybe all this effort to avoid Box<dyn Iterator<...>> was
    // premature optimisation... profile this and see if we should just use
    // dynamic dispatch instead for the readability boost.
    pub fn as_strs(&self) -> impl Iterator<Item = crate::Result<Cow<str>>> {
        enum CommandSegmentAsStrsIter<'a> {
            SplitWhitespace(
                std::iter::Map<std::str::SplitWhitespace<'a>, fn(&str) -> crate::Result<Cow<str>>>,
            ),
            Literally(std::iter::Once<crate::Result<Cow<'a, str>>>),
            Env(std::iter::Once<crate::Result<Cow<'a, str>>>),
        }

        impl<'a> Iterator for CommandSegmentAsStrsIter<'a> {
            type Item = crate::Result<Cow<'a, str>>;

            fn next(&mut self) -> Option<Self::Item> {
                match self {
                    CommandSegmentAsStrsIter::SplitWhitespace(iter) => iter.next(),
                    CommandSegmentAsStrsIter::Literally(iter) => iter.next(),
                    CommandSegmentAsStrsIter::Env(iter) => iter.next(),
                }
            }
        }

        match self {
            CommandSegment::SplitWhitespace(s) => CommandSegmentAsStrsIter::SplitWhitespace(
                s.split_whitespace().map(|s| Ok(Cow::Borrowed(s))),
            ),
            CommandSegment::Literally(s) => {
                CommandSegmentAsStrsIter::Literally(std::iter::once(Ok(Cow::Borrowed(s))))
            }
            CommandSegment::Env { name, if_unset } => {
                CommandSegmentAsStrsIter::Env(std::iter::once(match std::env::var(name) {
                    Ok(s) => Ok(Cow::Owned(s)),
                    Err(VarError::NotPresent) => match if_unset {
                        EnvUnsetHandler::Error => {
                            Err(ExecuteError::UnsetEnvVar(Box::new(name.clone())))
                        }
                        EnvUnsetHandler::ReplaceWith(s) => Ok(Cow::Borrowed(s)),
                        EnvUnsetHandler::Panic => {
                            panic!("{}", ExecuteError::UnsetEnvVar(Box::new(name.clone())))
                        }
                    },
                    Err(VarError::NotUnicode(_)) => todo!(),
                }))
            }
        }
    }
}
